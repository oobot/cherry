use sqlx::{Encode, Type};
use sqlx::Sqlite;

use crate::arguments::sqlite::SqliteArguments;
use crate::database::AboutDatabase;

impl<'a> AboutDatabase<'a, Sqlite, SqliteArguments<'a>> for Sqlite {

    fn arguments() -> SqliteArguments<'a> {
        SqliteArguments::new()
    }

    // fn add<T>(&mut self, v: T) where T: Encode<'a, Sqlite> + Type<Sqlite> + Send + 'a {
    //     self.add(v);
    // }
}