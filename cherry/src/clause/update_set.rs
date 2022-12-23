use sqlx::{Database, Encode, Type};

use crate::provider::Provider;
use crate::sql::update_set_clause::UpdateSetSection;

pub trait UpdateSet<'a, DB>: Provider<'a, DB> + Sized where DB: Database {

    fn set<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.sql_builder().add_update_section(UpdateSetSection::SetValue(c));
        self
    }

    fn set_column(mut self, c: &'a str) -> Self {
        self.sql_builder().add_update_section(UpdateSetSection::SetColumn(c));
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {


    }
}
