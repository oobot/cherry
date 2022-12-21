use sqlx::Database;

use crate::clause::set_value::SetValue;
use crate::sql::set_clause::SetSection;

pub trait SetColumn<'a, DB>: SetValue<'a, DB> + Sized where DB: Database {

    fn set_column(mut self, c: &'a str) -> Self {
        self.add_set_section(SetSection::SetColumn(c));
        self
    }

}