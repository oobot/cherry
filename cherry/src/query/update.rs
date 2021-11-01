use std::any::TypeId;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, impl_tx, impl_update_set, impl_where, Result};
use crate::adapt::query_result::QueryResult;
use crate::adapt::transaction::Transaction;
use crate::query::{self, Data};
use crate::query::query_builder::QueryBuilder;

pub struct Update<'a> {
    pub(crate) query: QueryBuilder<'a>,
}

impl<'a> Update<'a> {

    pub(crate) fn new<T: Cherry>(datasource: TypeId) -> Self {
        Self {
            query: QueryBuilder::new::<T>(datasource, SqlBuilder::update_table(T::table()))
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
    impl_update_set!(sqlx::MySql);
    #[cfg(feature = "postgres")]
    impl_update_set!(sqlx::Postgres);
    #[cfg(feature = "sqlite")]
    impl_update_set!(sqlx::Sqlite);
    #[cfg(feature = "mssql")]
    impl_update_set!(sqlx::Mssql);

    #[cfg(feature = "mysql")]
    impl_where!(sqlx::MySql);
    #[cfg(feature = "postgres")]
    impl_where!(sqlx::Postgres);
    #[cfg(feature = "sqlite")]
    impl_where!(sqlx::Sqlite);
    #[cfg(feature = "mssql")]
    impl_where!(sqlx::Mssql);
}

#[macro_export]
macro_rules! impl_update_set {
    ($db: ty) => {
        pub fn set<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.set_ref(f, v);
            self
        }

        pub fn set_ref<S, V>(&mut self, f: S, v: V) -> &Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.sql_builder.set(f, '?');
            self.query.arguments.add(v);
            self
        }
    }
}