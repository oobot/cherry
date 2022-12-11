use sqlx::{Encode, IntoArguments, Sqlite, Type};
use crate::arguments::Arguments;
use crate::sqlx::database::HasArguments;

pub struct SqliteArguments<'a>(pub(crate) sqlx::sqlite::SqliteArguments<'a>);

impl<'a> SqliteArguments<'a> {
    pub fn new() -> Self {
        Self(sqlx::sqlite::SqliteArguments::default())
    }

    pub fn add<T: Encode<'a, Sqlite> + Type<Sqlite> + Send + 'a>(&mut self, v: T) -> &mut Self {
        sqlx::Arguments::add(&mut self.0, v);
        // use sqlx::Arguments;
        // self.0.add(v);
        self
    }
}

impl<'a> Arguments<'a, Sqlite> for SqliteArguments<'a> {

    fn new() -> Self {
        Self::new()
    }

    fn add<T>(&mut self, v: T) where T: Encode<'a, Sqlite> + Type<Sqlite> + Send + 'a {
        self.add(v);
    }
}

impl<'a> IntoArguments<'a, Sqlite> for SqliteArguments<'a> {
    fn into_arguments(self) -> <Sqlite as HasArguments<'a>>::Arguments {
        self.0
    }
}