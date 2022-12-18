use sqlx::{Database, Encode, Type};
use sqlx::database::HasArguments;

#[cfg(feature = "mysql")]
pub use mysql::*;
#[cfg(feature = "postgres")]
pub use postgres::*;
#[cfg(feature = "sqlite")]
pub use sqlite::*;

use crate::arguments::Arguments;
use crate::query_builder::TargetQuery;

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "sqlite")]
mod sqlite;

pub trait AboutDatabase<'a, DB, ARGS>
    where
        DB: Database,
        ARGS: Arguments<'a, DB> + Sized + Send {

    fn arguments() -> ARGS;

    fn target() -> TargetQuery;
}

