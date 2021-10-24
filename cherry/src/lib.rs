// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde_derive;

pub use cherry::Cherry;
pub use mysql::template::MySqlTemplate;

pub use rows::WrapRows;
pub use arguments::{Arguments, WrapArguments, MySqlArguments, PgArguments,
                    SqliteArguments, MssqlArguments};
pub mod pools;
pub mod config;

pub(crate) mod arguments;
pub(crate) mod cherry;
pub(crate) mod rows;
pub(crate) mod mysql;
pub(crate) mod mssql;
pub(crate) mod postgres;
pub(crate) mod sqlite;
