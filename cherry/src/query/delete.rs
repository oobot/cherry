use std::any::TypeId;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, impl_tx, impl_where, Result};
use crate::adapt::query_result::QueryResult;
use crate::adapt::transaction::Transaction;
use crate::query::{self, Data};
use crate::query::enhance::query::Query;

pub struct Delete<'a> {
    pub(crate) query: Query<'a>,
}

impl<'a> Delete<'a> {
    pub(crate) fn new<T: Cherry>(datasource: TypeId) -> Self {
        Self {
            query: Query::new::<T>(datasource, SqlBuilder::delete_from(T::table()))
        }
    }

    pub async fn execute(self) -> Result<QueryResult>  {
        let data = Data {
            datasource: self.query.datasource,
            sql: self.query.sql_builder.sql()?,
            arguments: self.query.arguments,
            tx: self.query.tx
        };
        query::execute(data).await
    }

    impl_tx!();

    #[cfg(feature = "mysql")]
    impl_where!(sqlx::MySql);
    #[cfg(feature = "postgres")]
    impl_where!(sqlx::Postgres);
    #[cfg(feature = "sqlite")]
    impl_where!(sqlx::Sqlite);
    #[cfg(feature = "mssql")]
    impl_where!(sqlx::Mssql);

}
