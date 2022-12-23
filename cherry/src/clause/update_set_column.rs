use sqlx::Database;

use crate::clause::set_value::UpdateSet;
use crate::sql::set_clause::UpdateSetSection;

pub trait SetColumn<'a, DB>: UpdateSet<'a, DB> + Sized where DB: Database {

    fn set_column(mut self, c: &'a str) -> Self {
        self.add_set_section(UpdateSetSection::SetColumn(c));
        self
    }

}