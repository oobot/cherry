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
    sql_builder: Option<SqlBuilder>,
    replace: Option<(String, String)>,
    sql: Option<String>,
    tx: Option<&'a mut Transaction<'a>>,
    tx_auto: bool,
    arguments: Arguments<'a>,
}

// #[allow(dead_code)]
impl<'a> Insert<'a> {
    fn new(ds: TypeId) -> Self {
        Self {
            ds, sql_builder: None, replace: None, sql: None, tx: None, tx_auto: false,
            arguments: Arguments::new()
        }
    }

    fn sql_builder<T>(size: usize) -> SqlBuilder where T: Cherry {
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
        t.sql_builder = Some(Self::sql_builder::<T>(1));
        v.arguments(&mut t.arguments);
        t
    }

    pub(crate) fn insert_bulk<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new(ds);
        t.sql_builder = Some(Self::sql_builder::<T>(v.len()));
        v.iter().for_each(|v| v.arguments(&mut t.arguments) );
        t
    }

    pub(crate) fn insert_ignore<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new(ds);
        t.sql_builder = Some(Self::sql_builder::<T>(v.len()));
        t.replace = Some(("INSERT".into(), "INSERT IGNORE".into()));
        v.iter().for_each(|v| v.arguments(&mut t.arguments) );
        t
    }

    pub(crate) fn insert_replace<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new(ds);
        t.sql_builder = Some(Self::sql_builder::<T>(v.len()));
        t.replace = Some(("INSERT INTO".into(), "INSERT REPLACE INTO".into()));
        v.iter().for_each(|v| v.arguments(&mut t.arguments) );
        t
    }

    pub fn tx(mut self, tx: &'a mut Transaction<'a>) -> Self {
        self.tx_ref(tx);
        self
    }

    fn tx_ref(&mut self, tx: &'a mut Transaction<'a>) -> &Self {
        self.tx = Some(tx);
        self
    }

    pub fn tx_auto(mut self) -> Self {
        self.tx_auto_ref();
        self
    }

    fn tx_auto_ref(&mut self) -> &Self {
        self.tx_auto = true;
        self
    }

    fn build_sql(&self) -> Result<String> {
        let mut sql = self.sql_builder.as_ref()
            .ok_or_else(|| anyhow!("SqlBuilder is empty. This wasn’t supposed to happen."))?
            .sql()?;
        if let Some((src, target)) = &self.replace {
            sql = sql.replacen(src.as_str(), target.as_str(), 1);
        }
        Ok(sql)
    }

    pub async fn execute(self) -> Result<QueryResult>  {
        if self.tx_auto && self.tx.is_some() {
            return Err(anyhow!("Manual and automatic transactions can not exist simultaneously."));
        }

        let sql = match self.sql {
            Some(sql) => sql,
            _ => self.build_sql()?,
        };
        let sql = sql.as_str();
        let arguments = self.arguments.inner;

        let result = if let Some(tx) = self.tx { // Manual transaction.
            sqlx::query_with(sql, arguments).execute(&mut tx.inner).await?
        } else if self.tx_auto { // Auto transaction.
            let mut tx = pool::get(self.ds)?.inner.begin().await?;
            let result = sqlx::query_with(sql, arguments).execute(&mut tx).await?;
            tx.commit().await?;
            result
        } else { // No transaction.
            let pool = &pool::get(self.ds)?.inner;
            sqlx::query_with(sql, arguments).execute(pool).await?
        };

        Ok(QueryResult::from(result))
    }

}

pub struct InsertUpdate<'a> {
    insert: Insert<'a>,
    fields: Vec<String>,
}

impl<'a> InsertUpdate<'a> {

    fn new(ds: TypeId) -> Self {
        Self { insert: Insert::new(ds), fields: vec![] }
    }

    pub(crate) fn insert_update<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new(ds);
        t.insert.sql_builder = Some(Insert::sql_builder::<T>(v.len()));
        v.iter().for_each(|v| v.arguments(&mut t.insert.arguments) );
        t
    }

    pub fn field<T: AsRef<str>>(mut self, f: T) -> Self {
        self.field_ref(f);
        self
    }

    pub fn field_ref<T: AsRef<str>>(&mut self, f: T) -> &Self {
        self.fields.push(f.as_ref().to_owned());
        self
    }

    pub fn fields<T: AsRef<str>>(mut self, f: &[T]) -> Self {
        self.fields_ref(f);
        self
    }

    pub fn fields_ref<T: AsRef<str>>(&mut self, f: &[T]) -> &Self {
        f.iter().for_each(|f| {
            self.fields.push(f.as_ref().to_owned());
        });
        self
    }

    pub fn tx(mut self, tx: &'a mut Transaction<'a>) -> Self {
        self.insert.tx_ref(tx);
        self
    }

    pub fn tx_auto(mut self) -> Self {
        self.insert.tx_auto_ref();
        self
    }

    pub async fn execute(mut self) -> Result<QueryResult>  {
        if self.fields.is_empty() {
            return Err(anyhow!("Empty update fields."));
        }
        let insert = self.insert.build_sql()?.strip_suffix(";")
            .ok_or(anyhow!("Empty sql. This wasn’t supposed to happen."))?
            .to_owned();
        let update = self.fields.iter().map(|x| format!("{0} = new.{0}, ", x))
            .collect::<String>()
            .strip_suffix(",")
            .ok_or(anyhow!("Empty sql. This wasn’t supposed to happen."))?
            .to_owned();

        self.insert.sql = Some(format!("{} AS new ON DUPLICATE KEY UPDATE {};", insert, update));
        self.insert.execute().await
    }

}
