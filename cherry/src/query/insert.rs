use std::any::TypeId;

use sql_builder::SqlBuilder;

use crate::{Cherry, impl_tx, Result};
use crate::adapt::query_result::QueryResult;
use crate::adapt::transaction::Transaction;
use crate::query::{self, Data};
use crate::query::query_builder::QueryBuilder;

pub struct Insert<'a> {
    pub(crate) query: QueryBuilder<'a>,
    pub(crate) columns: Vec<&'static str>,
    pub(crate) replace: Option<(String, String)>,
    pub(crate) size: usize,
}

impl<'a> Insert<'a> {

    pub(crate) fn new<T>(datasource: TypeId) -> Self where T: Cherry {
        let query = QueryBuilder::new::<T>(datasource, SqlBuilder::insert_into(T::table()));
        Self { query, columns: T::columns(), replace: None, size: 0 }
    }

    pub(crate) fn insert<T>(datasource: TypeId, v: &'a T) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = 1;
        v.arguments(&mut t.query.arguments);
        t
    }

    pub(crate) fn insert_bulk<T>(datasource: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = v.len();
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    pub(crate) fn insert_ignore<T>(datasource: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = v.len();
        t.replace = Some(("INSERT".into(), "INSERT IGNORE".into()));
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    pub(crate) fn insert_replace<T>(datasource: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = v.len();
        t.replace = Some(("INSERT INTO".into(), "INSERT REPLACE INTO".into()));
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    pub async fn execute(mut self) -> Result<QueryResult>  {
        let holders = vec!["?"; self.columns.len()];
        self.query.sql_builder.fields(self.columns.as_slice());
        (0..self.size).for_each(|_| {
            self.query.sql_builder.values(holders.as_slice());
        });
        let mut sql = self.query.sql_builder.sql()?;
        if let Some((src, target)) = &self.replace {
            sql = sql.replacen(src.as_str(), target.as_str(), 1);
        }

        let data = Data {
            datasource: self.query.datasource,
            sql,
            arguments: self.query.arguments,
            tx: self.query.tx
        };
        query::execute(data).await
    }

    impl_tx!();
}
