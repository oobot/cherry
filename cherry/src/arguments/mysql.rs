use sqlx::{Encode, MySql, Type};

use crate::arguments::Arguments;
use crate::sqlx::database::HasArguments;

pub struct MySqlArguments(pub(crate) sqlx::mysql::MySqlArguments);

impl MySqlArguments {
    pub fn new() -> Self {
        Self(sqlx::mysql::MySqlArguments::default())
    }

    pub fn add<'a, T: Encode<'a, MySql> + Type<MySql>>(&mut self, v: T) -> &mut Self {
        sqlx::Arguments::add(&mut self.0, v);
        // use sqlx::Arguments;
        // self.0.add(v);
        self
    }
}

impl<'a> Arguments<'a, MySql> for MySqlArguments {
    fn new() -> Self {
        Self::new()
    }

    fn raw(self) -> <MySql as HasArguments<'a>>::Arguments {
        self.0
    }

    fn add<T>(&mut self, v: T) where T: Encode<'a, MySql> + Type<MySql> + Send + 'a {
        self.add(v);
    }
}