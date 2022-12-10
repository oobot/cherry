use sqlx::{Encode, Type};
use sqlx::MySql;

use crate::AboutDatabase;
use crate::arguments::mysql::MySqlArguments;
use crate::database::AboutDatabase;

impl<'a> AboutDatabase<'a, MySql, MySqlArguments> for MySql {

    fn arguments() -> MySqlArguments {
        MySqlArguments::new()
    }

    /*fn add<T>(&mut self, v: T) where T: Encode<'a, MySql> + Type<MySql> + Send + 'a {
        self.add(v);
        // (self as MySqlArguments).add(v);
    }*/
}