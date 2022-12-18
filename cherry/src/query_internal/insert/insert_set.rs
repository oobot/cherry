use sqlx::Database;

use crate::query_builder::set_clause::SetSection;
use crate::query_internal::set::UpdateSet;

pub trait InsertSet<'a, DB>: UpdateSet<'a, DB> + Sized where DB: Database {

    fn set_column<V>(mut self, c: &'a str) -> Self {
        self.add_set_section(SetSection::SetColumn(c));
        self
    }

}