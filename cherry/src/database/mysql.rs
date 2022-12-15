use sqlx::{Encode, Type};
use sqlx::MySql;

use crate::arguments::mysql::MySqlArguments;
use crate::database::AboutDatabase;
use crate::query_builder::TargetDatabase;

impl<'a> AboutDatabase<'a, MySql, MySqlArguments> for MySql {

    fn arguments() -> MySqlArguments {
        MySqlArguments::new()
    }

    fn database() -> TargetDatabase {
        TargetDatabase::MySql
    }
}