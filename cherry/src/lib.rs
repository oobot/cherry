#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub(crate) type Result<T> = std::result::Result<T, anyhow::Error>;
pub(crate) mod arguments;
pub(crate) mod row;
pub(crate) mod cherry;


// top-level
pub use {
    cherry::Cherry,
    arguments::Arguments,
    row::Row,
};

pub mod pools;
pub mod config;


pub mod error {
    pub use anyhow::Error;
}

// re-export sqlx
pub use sqlx::{Database, Encode, Decode};
pub mod types {
    pub use sqlx::types::{Type, Json, Uuid};
}

pub mod mysql {
    pub use sqlx::mysql::MySql;
}

// pub mod postgres {
//     pub use sqlx::postgres::Postgres;
// }
//
// pub mod sqlite {
//     pub use sqlx::sqlite::Sqlite;
// }
//
// pub mod mssql {
//     pub use sqlx::mssql::Mssql;
// }
