#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub use {
    anyhow::Error,
    cherry::Cherry,
};

pub mod cherry;
pub mod database;
pub mod pool;
pub mod arguments;
pub mod row;
pub mod query;
pub mod query_builder;

pub mod sqlx {
    pub use sqlx::*;
}


#[cfg(not(any(feature = "mysql", feature = "postgres", feature = "sqlite")))]
compile_error!("one of the features ['mysql', 'postgres', 'sqlite'] must be enabled");
