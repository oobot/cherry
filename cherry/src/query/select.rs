use std::any::TypeId;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, connection, impl_tx, impl_where, Result};
use crate::adapt::query_result::QueryResult;
use crate::adapt::row::Row;
use crate::adapt::transaction::Transaction;
use crate::query::{self, Data};
use crate::query::query_builder::QueryBuilder;

pub struct Select<'a> {
    pub(crate) query: QueryBuilder<'a>,
}

impl<'a> Select<'a> {

    pub(crate) fn new<T: Cherry>(datasource: TypeId) -> Self {
        Self {
            query: QueryBuilder::new::<T>(datasource, SqlBuilder::select_from(T::table()))
        }
    }

    pub fn field<S: ToString>(mut self, f: S) -> Self {
        self.query.sql_builder.field(f);
        self
    }

    pub fn fields<S: ToString>(mut self, fields: &[S]) -> Self {
        self.query.sql_builder.fields(fields);
        self
    }

    pub fn order_asc<S: ToString>(mut self, f: S) -> Self {
        self.query.sql_builder.order_asc(f);
        self
    }

    pub fn order_desc<S: ToString>(mut self, f: S) -> Self {
        self.query.sql_builder.order_desc(f);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.query.sql_builder.limit(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.query.sql_builder.offset(offset);
        self
    }

    pub async fn fetch<T>(self) -> Result<Option<T>> where T: Cherry {
        let row = sqlx::query_with(
            self.query.sql_builder.sql()?.as_str(),
            self.query.arguments.inner
        ).fetch_optional(&connection::get(self.query.datasource)?.inner).await?;
        match row {
            Some(row) => Ok(Some(T::from_row(&Row(row))?)),
            _ => Ok(None)
        }
    }

    pub async fn fetch_all<T>(self) -> Result<Vec<T>> where T: Cherry {
        let rows = sqlx::query_with(
            self.query.sql_builder.sql()?.as_str(),
            self.query.arguments.inner
        ).fetch_all(&connection::get(self.query.datasource)?.inner).await?;
        let mut vec = Vec::with_capacity(rows.len());
        for row in rows {
            vec.push(T::from_row(&Row(row))?);
        }
        Ok(vec)
    }

    #[cfg(feature = "mysql")]
    impl_where!(sqlx::MySql);
    #[cfg(feature = "postgres")]
    impl_where!(sqlx::Postgres);
    #[cfg(feature = "sqlite")]
    impl_where!(sqlx::Sqlite);
    #[cfg(feature = "mssql")]
    impl_where!(sqlx::Mssql);

}
