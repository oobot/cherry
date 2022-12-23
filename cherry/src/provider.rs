use sqlx::{Database, Encode, Type};

use crate::sql::builder::SqlBuilder;

pub trait Provider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn sql_builder(&mut self) -> &mut SqlBuilder<'a>;

}
