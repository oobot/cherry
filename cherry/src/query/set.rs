use sqlx::{Database, Encode, Type};

use crate::query::provider::SetProvider;
use crate::query_builder::set_clause::SetSection;

pub trait Set<'a, DB>: SetProvider<'a, DB> + Sized where DB: Database {

    fn set<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_set_section(SetSection::SetValue(c));
        self
    }

    fn set_column<V>(mut self, c: &'a str) -> Self {
        self.add_set_section(SetSection::SetColumn(c));
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
