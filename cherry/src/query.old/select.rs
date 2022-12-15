use std::marker::PhantomData;

use log::debug;
use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, connection, gen_where};
use crate::query::query_builder::QueryBuilder;
use crate::types::{Database, Result};

pub struct Select<'a, T> {
    _keep: PhantomData<T>,
    pub(crate) query: QueryBuilder<'a>,
}

impl<'a, T> Select<'a, T> where T: Cherry {

    pub(crate) fn new(ds: &'a str) -> Self {
        Self {
            _keep: PhantomData,
            query: QueryBuilder::new::<T>(ds, SqlBuilder::select_from(T::table()))
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

    pub fn fields_all(mut self) -> Self {
        self.query.sql_builder.fields(&T::columns());
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

    gen_where!();

    pub async fn fetch(self) -> Result<Option<T>> {
        let sql = self.query.sql_builder.sql()?;
        debug!("{}", sql);
        let row = sqlx::query_with(
            sql.as_str(),
            self.query.arguments
        ).fetch_optional(connection::get(self.query.datasource)?).await?;
        match row {
            Some(row) => Ok(Some(T::from_row(&row)?)),
            _ => Ok(None)
        }
    }

    pub async fn fetch_all(self) -> Result<Vec<T>> {
        let sql = self.query.sql_builder.sql()?;
        debug!("{}", sql);
        let rows = sqlx::query_with(
            sql.as_str(),
            self.query.arguments
        ).fetch_all(connection::get(self.query.datasource)?).await?;
        let mut vec = Vec::with_capacity(rows.len());
        for row in rows {
            vec.push(T::from_row(&row)?);
        }
        Ok(vec)
    }

}
