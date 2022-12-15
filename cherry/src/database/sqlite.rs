use sqlx::{Encode, Type};
use sqlx::Sqlite;

use crate::arguments::sqlite::SqliteArguments;
use crate::database::AboutDatabase;
use crate::query_builder::QueryDatabase;

impl<'a> AboutDatabase<'a, Sqlite, SqliteArguments<'a>> for Sqlite {

    fn arguments() -> SqliteArguments<'a> {
        SqliteArguments::new()
    }

    fn database() -> QueryDatabase {
        QueryDatabase::Sqlite
    }
}