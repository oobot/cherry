// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub use {
    cherry::Cherry,
    adapter::AnyArguments,
    adapter::AnyRow,
    anyhow::Error,
    // connection::PoolConfig,
    // datasource::DataSource,
    // types::Transaction,
};

pub(crate) mod cherry;
mod adapter;
// pub(crate) mod connection;
// pub(crate) mod query;
// pub(crate) mod datasource;

// pub mod types;

// pub mod anyhow {
//     pub use anyhow::*;
// }

pub mod sqlx {
    pub use sqlx::*;
}


/*pub mod sqlx {
    pub use sqlx::{Arguments, Database, Decode, Encode, Row, types::Type};
    #[cfg(feature = "json")]
    pub use sqlx::types::Json;
    #[cfg(feature = "uuid")]
    pub use sqlx::types::Uuid;

// #[cfg(feature = "mysql")]
    // pub use sqlx::mysql::{MySql, MySqlArguments, MySqlQueryResult, MySqlRow};
    // #[cfg(feature = "postgres")]
    // pub use sqlx::postgres::{PgArguments, PgQueryResult, PgRow, Postgres};
    // #[cfg(feature = "sqlite")]
    // pub use sqlx::sqlite::{Sqlite, SqliteArguments, SqliteQueryResult, SqliteRow};
    // #[cfg(feature = "mssql")]
    // pub use sqlx::mssql::{Mssql, MssqlArguments, MssqlQueryResult, MssqlRow};
}*/

#[cfg(not(any(feature = "mysql", feature = "postgres", feature = "sqlite", feature = "mssql")))]
compile_error!("one of the features ['mysql', 'postgres', 'sqlite', 'mssql'] must be enabled");
