use sqlx::{Database, Encode, Type};

use crate::query_builder::set_clause::SetSection;
use crate::query_internal::provider::SetProvider;

pub trait UpdateSet<'a, DB>: SetProvider<'a, DB> + Sized where DB: Database {

    fn set<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_set_section(SetSection::SetValue(c));
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
