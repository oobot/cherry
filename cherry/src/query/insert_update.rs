use std::any::TypeId;

use anyhow::anyhow;
use sql_builder::SqlBuilder;

use crate::{Cherry, impl_tx, Result};
use crate::adapt::query_result::QueryResult;
use crate::adapt::transaction::Transaction;
use crate::query::{self, Data};
use crate::query::enhance::query::Query;

pub struct InsertUpdate<'a> {
    pub(crate) query: Query<'a>,
    pub(crate) columns: Vec<&'static str>,
    pub(crate) size: usize,
    pub(crate) fields: Vec<String>,
}

impl<'a> InsertUpdate<'a> {

    fn new<T>(datasource: TypeId) -> Self where T: Cherry {
        Self {
            query: Query::new::<T>(datasource, SqlBuilder::insert_into(T::table())),
            columns: T::columns(),
            size: 0,
            fields: vec![]
        }
    }

    pub(crate) fn insert_update<T>(datasource: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(datasource);
        t.size = v.len();
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
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

    pub async fn execute(mut self) -> Result<QueryResult>  {
        if self.fields.is_empty() {
            return Err(anyhow!("Empty update fields."));
        }

        let holders = vec!["?"; self.columns.len()];
        self.query.sql_builder.fields(self.columns.as_slice());
        (0..self.size).for_each(|_| {
            self.query.sql_builder.values(holders.as_slice());
        });

        let insert = self.query.sql_builder.sql()?.strip_suffix(";")
            .ok_or(anyhow!("Empty sql. This wasn’t supposed to happen."))?
            .to_owned();
        let update = self.fields.iter().map(|x| format!("{0} = new.{0}, ", x))
            .collect::<String>()
            .strip_suffix(",")
            .ok_or(anyhow!("Empty sql. This wasn’t supposed to happen."))?
            .to_owned();

        let data = Data {
            datasource: self.query.datasource,
            sql: format!("{} AS new ON DUPLICATE KEY UPDATE {};", insert, update),
            arguments: self.query.arguments,
            tx: self.query.tx
        };

        query::execute(data).await
    }

    impl_tx!();

}