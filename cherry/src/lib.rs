// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub use arguments::{Arguments, MssqlArguments, MySqlArguments, PgArguments,
                    SqliteArguments, WrapArguments};
pub use cherry::Cherry;
pub use cherry_err as cherry;
pub use error::Error;
pub use mysql::template::MySqlTemplate;
pub mod row;
pub mod pools;
pub mod config;

pub(crate) mod error;
pub(crate) mod arguments;
pub(crate) mod cherry;

pub(crate) mod mysql;
pub(crate) mod mssql;
pub(crate) mod postgres;
pub(crate) mod sqlite;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;
