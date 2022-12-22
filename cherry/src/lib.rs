// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub use {
    anyhow::Error,
    cherry::Cherry,
    cherry_derive::Cherry,
    pool::PoolOptions,
    query::Query,
    executor::QueryExecutor,
};

pub mod clause;
pub(crate) mod cherry;
pub(crate) mod pool;
pub(crate) mod query;
pub(crate) mod executor;
pub(crate) mod provider;
pub(crate) mod sql;

#[cfg(feature = "sqlite")]
pub mod sqlite {
    pub use crate::pool::sqlite::SqlitePool;
}

#[cfg(feature = "postgres")]
pub mod postgres {
    pub use crate::pool::postgres::PgPool;
}

#[cfg(feature = "mysql")]
pub mod mysql {
    pub use crate::pool::mysql::MySqlPool;
}

pub mod sqlx {
    pub use sqlx::*;
}

#[cfg(not(any(feature = "mysql", feature = "postgres", feature = "sqlite")))]
compile_error!("at least one of the features ['mysql', 'postgres', 'sqlite'] must be enabled");
