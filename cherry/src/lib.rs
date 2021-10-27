// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub(crate) type Result<T> = std::result::Result<T, anyhow::Error>;
pub(crate) mod arguments;
pub(crate) mod row;
pub(crate) mod cherry;


// top-level
pub use {
    cherry::Cherry,
    arguments::Arguments,
    row::Row,
    sql_builder::SqlBuilder as Sql,
};

pub mod config;
pub mod datasource;


pub mod error {
    pub use anyhow::Error;
}

// re-export sqlx
pub use sqlx::{Database, Encode, Decode};
pub mod types {
    pub use sqlx::types::Type;
    #[cfg(feature = "json")]
    pub use sqlx::types::Json;
    #[cfg(feature = "uuid")]
    pub use sqlx::types::Uuid;
}

#[cfg(feature = "mysql")]
pub mod mysql {
    pub use sqlx::mysql::MySql;
}

#[cfg(feature = "postgres")]
pub mod postgres {
    pub use sqlx::postgres::Postgres;
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    pub use sqlx::sqlite::Sqlite;
}

#[cfg(feature = "mssql")]
pub mod mssql {
    pub use sqlx::mssql::Mssql;
}
