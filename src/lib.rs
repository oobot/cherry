// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub(crate) mod cherry;
pub(crate) mod datasource;
pub(crate) mod query;

pub use {
    cherry::Cherry,
    datasource::DataSource,
};

pub mod types;
pub mod connection;

pub mod error {
    pub use anyhow::Error;
}

pub mod sqlx {
    pub use sqlx::{Database, Decode, Encode, Arguments, Row};
    // pub use sqlx::database::HasArguments;

    #[cfg(feature = "mysql")]
    pub use sqlx::mysql::{MySql, MySqlArguments, MySqlQueryResult, MySqlRow};
    #[cfg(feature = "postgres")]
    pub use sqlx::postgres::{PgArguments, PgQueryResult, PgRow, Postgres};
    #[cfg(feature = "sqlite")]
    pub use sqlx::sqlite::{Sqlite, SqliteArguments, SqliteQueryResult, SqliteRow};
    #[cfg(feature = "mssql")]
    pub use sqlx::mssql::{Mssql, MssqlArguments, MssqlQueryResult, MssqlRow};

    pub use sqlx::types::Type;
    #[cfg(feature = "json")]
    pub use sqlx::types::Json;
    #[cfg(feature = "uuid")]
    pub use sqlx::types::Uuid;
}

/*
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
*/

#[cfg(not(any(feature = "mysql", feature = "postgres", feature = "sqlite", feature = "mssql")))]
compile_error!("one of the features ['mysql', 'postgres', 'sqlite', 'mssql'] must be enabled");
