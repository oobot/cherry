use sqlx::{Encode, IntoArguments, Postgres, Type};
use sqlx::database::HasArguments;

use crate::arguments::Arguments;

pub struct PgArguments(pub(crate) sqlx::postgres::PgArguments);

impl PgArguments {
    pub fn new() -> Self {
        Self(sqlx::postgres::PgArguments::default())
    }

    pub fn add<'a, T>(&mut self, v: T) -> &mut Self
        where T: Encode<'a, Postgres> + Type<Postgres> + Send + 'a {
        sqlx::Arguments::add(&mut self.0, v);
        // use sqlx::Arguments;
        // self.0.add(v);
        self
    }
}

impl<'a> Arguments<'a, Postgres> for PgArguments {

    fn new() -> Self {
        Self::new()
    }

    fn add<T>(&mut self, v: T) where T: Encode<'a, Postgres> + Type<Postgres> + Send + 'a {
        self.add(v);
    }
}

impl<'a> IntoArguments<'a, Postgres> for PgArguments {

    fn into_arguments(self) -> <Postgres as HasArguments<'a>>::Arguments {
        self.0
    }

}
