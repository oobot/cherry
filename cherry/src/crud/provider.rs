use sqlx::{Database, Encode, Type};

use crate::statement::end::EndStatement;
use crate::statement::r#where::WhereStatement;

pub trait WhereProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn where_statement(&mut self) -> &mut WhereStatement<'a>;
}

pub trait EndProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn end_statement(&mut self) -> &mut EndStatement<'a>;
}
