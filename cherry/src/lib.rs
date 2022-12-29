// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub use {
    anyhow::Error,
    cherry::Cherry,
    cherry_derive::Cherry,
    executor::QueryExecutor,
    query::Query,
    crate::sqlx::pool::Pool,
    crate::sqlx::pool::PoolOptions,
};

pub mod clause;
pub(crate) mod cherry;
pub(crate) mod query;
pub(crate) mod executor;
pub(crate) mod provider;
pub(crate) mod sql;

#[cfg(feature = "sqlite")]
pub mod sqlite {
    pub use sqlx::sqlite::{Sqlite, SqlitePool, SqlitePoolOptions};
}

#[cfg(feature = "postgres")]
pub mod postgres {
    pub use sqlx::postgres::{Postgres, PgPool, PgPoolOptions};
}

#[cfg(feature = "mysql")]
pub mod mysql {
    pub use sqlx::mysql::{MySql, MySqlPool, MySqlPoolOptions};
}

pub mod sqlx {
    pub use sqlx::*;
}

#[cfg(not(any(feature = "mysql", feature = "postgres", feature = "sqlite")))]
compile_error!("at least one of the features ['mysql', 'postgres', 'sqlite'] must be enabled");
