#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub(crate) type Result<T> = std::result::Result<T, anyhow::Error>;
pub(crate) mod cherry;
pub(crate) mod adapt;
pub(crate) mod query;
pub(crate) mod datasource;

// top-level
pub use {
    cherry::Cherry,
    datasource::DataSource,

    adapt::row::Row,
    adapt::arguments::Arguments,
    adapt::query_result::QueryResult,
    adapt::transaction::Transaction,

    query::insert::Insert,
    query::insert_update::InsertUpdate,
    query::update::Update,
    query::delete::Delete,
    query::select::Select,

    sqlx::{Database, Decode, Encode},
    sqlx::database::HasArguments,
};

pub mod connection;
pub mod error {
    pub use anyhow::Error;
}

pub mod types {
    pub use sqlx::types::Type;
    #[cfg(feature = "json")]
    pub use sqlx::types::Json;
    #[cfg(feature = "uuid")]
    pub use sqlx::types::Uuid;
}

#[cfg(feature = "mysql")]
pub mod mysql {
    pub use sqlx::mysql::{MySql, MySqlArguments, MySqlQueryResult, MySqlRow};
}

#[cfg(feature = "postgres")]
pub mod postgres {
    pub use sqlx::postgres::{PgArguments, PgQueryResult, PgRow, Postgres};
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    pub use sqlx::sqlite::{Sqlite, SqliteArguments, SqliteQueryResult, SqliteRow};
}

#[cfg(feature = "mssql")]
pub mod mssql {
    pub use sqlx::mssql::{Mssql, MssqlArguments, MssqlQueryResult, MssqlRow};
}
