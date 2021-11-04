use std::any::TypeId;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, connection, gen_execute, gen_where};
use crate::query::query_builder::QueryBuilder;
use crate::types::{Database, QueryResult, Result, Transaction};

pub struct Update<'a> {
    pub(crate) query: QueryBuilder<'a>,
}

impl<'a> Update<'a> {
    pub(crate) fn new<T: Cherry>(datasource: TypeId) -> Self {
        Self {
            query: QueryBuilder::new::<T>(datasource, SqlBuilder::update_table(T::table()))
        }
    }

    pub fn set<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type< Database> + Send + 'a
    {
        self.set_ref(f, v);
        self
    }

    pub fn set_ref<S, V>(&mut self, f: S, v: V) -> &Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type< Database> + Send + 'a
    {
        self.query.sql_builder.set(f, '?');
        self.query.add_arguments(v);
        self
    }

    gen_where!();

    fn build_sql(&mut self) -> Result<String> {
        Ok(self.query.sql_builder.sql()?)
    }

    gen_execute!();

}
