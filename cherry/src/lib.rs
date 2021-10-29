// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub(crate) type Result<T> = std::result::Result<T, anyhow::Error>;
pub(crate) mod cherry;
pub(crate) mod arguments;
pub(crate) mod row;
pub(crate) mod tx;
pub(crate) mod query_result;


// top-level
pub use {
    cherry::Cherry,
    arguments::Arguments,
    row::Row,
    sql_builder::SqlBuilder as Sql,
};

pub mod config;
pub mod datasource;
pub mod pool;
pub mod insert;



pub mod error {
    pub use anyhow::Error;
}

// re-export sqlx
pub use sqlx::{Database, Encode, Decode, Transaction};
pub use sqlx::database::HasArguments;
pub mod types {
    pub use sqlx::types::Type;
    #[cfg(feature = "json")]
    pub use sqlx::types::Json;
    #[cfg(feature = "uuid")]
    pub use sqlx::types::Uuid;
}

#[cfg(feature = "mysql")]
pub mod mysql {
    pub use sqlx::mysql::{MySql, MySqlArguments, MySqlRow, MySqlQueryResult};
}

#[cfg(feature = "postgres")]
pub mod postgres {
    pub use sqlx::postgres::{Postgres, PgArguments, PgRow, PgQueryResult};
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    pub use sqlx::sqlite::{Sqlite, SqliteArguments, SqliteRow, SqliteQueryResult};
}

#[cfg(feature = "mssql")]
pub mod mssql {
    pub use sqlx::mssql::{Mssql, MssqlArguments, MssqlRow, MssqlQueryResult};
}
