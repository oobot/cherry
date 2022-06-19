use log::debug;
use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, connection, gen_execute, gen_where};
use crate::query::query_builder::QueryBuilder;
use crate::types::{Database, QueryResult, Result, Transaction};

pub struct Delete<'a> {
    pub(crate) query: QueryBuilder<'a>,
}

impl<'a> Delete<'a> {

    pub(crate) fn new<T: Cherry>(ds: &'a str) -> Self {
        Self {
            query: QueryBuilder::new::<T>(ds, SqlBuilder::delete_from(T::table()))
        }
    }

    fn build_sql(&mut self) -> Result<String> {
        Ok(self.query.sql_builder.sql()?)
    }

    gen_where!();
    gen_execute!();

}
