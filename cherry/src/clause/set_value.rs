use sqlx::{Database, Encode, Type};

use crate::provider::SetValueProvider;
use crate::sql::set_clause::SetSection;

pub trait SetValue<'a, DB>: SetValueProvider<'a, DB> + Sized where DB: Database {

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
