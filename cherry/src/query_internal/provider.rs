use sqlx::{Database, Encode, Type};

use crate::query_builder::end::section::EndSection;
use crate::query_builder::set_clause::SetSection;
use crate::query_builder::where_clause::condition::Condition;

pub trait WhereProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn make_wrap(&mut self);

    fn take_wrap(&mut self) -> Vec<Condition<'a>>;

    fn add_where_condition(&mut self, c: Condition<'a>);
}

pub trait SetProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn add_set_section(&mut self, section: SetSection<'a>);
}

pub trait EndProvider<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn add_end_section(&mut self, section: EndSection<'a>);
}
