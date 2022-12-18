use sqlx::Database;

use crate::arguments::Arguments;
use crate::query_builder::TargetQuery;

#[cfg(feature = "postgres")]
pub(crate) mod postgres;
#[cfg(feature = "mysql")]
pub(crate) mod mysql;
#[cfg(feature = "sqlite")]
pub(crate) mod sqlite;

pub trait AboutDatabase<'a, DB, ARGS>
    where
        DB: Database,
        ARGS: Arguments<'a, DB> + Sized + Send {

    fn arguments() -> ARGS;

    fn target() -> TargetQuery;
}

