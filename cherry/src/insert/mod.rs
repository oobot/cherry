use std::any::TypeId;

use anyhow::anyhow;
use sql_builder::SqlBuilder;

use crate::{Cherry, Result};
use crate::arguments::Arguments;
use crate::pool;
use crate::query_result::QueryResult;
use crate::tx::Transaction;

pub struct Insert<'a> {
    ds: TypeId,
    sql: Option<SqlBuilder>,
    replace: Option<(String, String)>,
    tx: Option<&'a mut Transaction<'a>>,
    tx_auto: bool,
    arguments: Arguments<'a>,

}

#[allow(dead_code)]
impl<'a> Insert<'a> {
    fn new(ds: TypeId) -> Self {
        Self {ds, sql: None, replace: None, tx: None, tx_auto: false, arguments: Arguments::new()}
    }

    fn bulk_sql<T>(size: usize) -> SqlBuilder where T: Cherry {
        let columns = T::columns();
        let holders = vec!["?"; columns.len()];
        let mut sql = SqlBuilder::insert_into(T::table());
        sql.fields(columns.as_slice());
        (0..size).for_each(|_| {
            sql.values(holders.as_slice());
        });
        sql
    }

    pub(crate) fn insert<T>(ds: TypeId, v: &'a T) -> Self where T: Cherry {
        let mut t = Self::new(ds);
        t.sql = Some(Self::bulk_sql::<T>(1));
        v.arguments(&mut t.arguments);
        t
    }

    pub(crate) fn insert_bulk<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new(ds);
        t.sql = Some(Self::bulk_sql::<T>(v.len()));
        v.iter().for_each(|v| v.arguments(&mut t.arguments) );
        t
    }

    pub(crate) fn insert_ignore<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new(ds);
        t.sql = Some(Self::bulk_sql::<T>(v.len()));
        t.replace = Some(("INSERT".into(), "INSERT IGNORE".into()));
        v.iter().for_each(|v| v.arguments(&mut t.arguments) );
        t
    }

    pub(crate) fn insert_replace<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new(ds);
        t.sql = Some(Self::bulk_sql::<T>(v.len()));
        t.replace = Some(("INSERT INTO".into(), "INSERT REPLACE INTO".into()));
        v.iter().for_each(|v| v.arguments(&mut t.arguments) );
        t
    }

    pub fn tx(mut self, tx: &'a mut Transaction<'a>) -> Self {
        self.tx = Some(tx);
        self
    }

    pub fn tx_auto(mut self) -> Self {
        self.tx_auto = true;
        self
    }

    pub async fn execute(self) -> Result<QueryResult>  {
        if self.tx_auto && self.tx.is_some() {
            return Err(anyhow!("Manual and automatic transactions can not exist simultaneously."));
        }

        let arguments = self.arguments.inner;
        let mut sql = self.sql.expect("Should not be happened.").sql()?;
        if let Some((src, target)) = self.replace {
            sql = sql.replacen(src.as_str(), target.as_str(), 1);
        }

        let result = if let Some(tx) = self.tx { // Manual transaction.
            sqlx::query_with(sql.as_str(), arguments).execute(&mut tx.inner).await?
        } else if self.tx_auto { // Auto transaction.
            let mut tx = pool::get(self.ds)?.begin().await?;
            let result = sqlx::query_with(sql.as_str(), arguments)
                .execute(&mut tx)
                .await?;
            tx.commit().await?;
            result
        } else { // No transaction.
            let pool = pool::get(self.ds)?;
            sqlx::query_with(sql.as_str(), arguments).execute(pool).await?
        };

        Ok(QueryResult::from(result))
    }

}



pub struct InsertUpdate {
    // Include Insert
}