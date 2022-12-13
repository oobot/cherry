use sqlx::{Database, Encode, Type};

use crate::sql::condition::Condition;
use crate::sql::filter_statement::FilterStatement;

pub trait Filter<'a, DB>: Sized where DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a;

    fn filter(&mut self) -> &mut FilterStatement<'a>;

    // closure wrap conditions
    fn and<F>(mut self, mut f: F) -> Self where F: FnMut(&mut Self) -> &mut Self {
        self.filter().make_temp();
        // closure will add to temp conditions
        f(&mut self);
        let conditions = self.filter().take_temp();
        self.filter().add_condition(Condition::And(conditions));
        self
    }

    fn or<F>(mut self, mut f: F) -> Self where F: FnMut(&mut Self) -> &mut Self {
        self.filter().make_temp();
        f(&mut self); // closure will add to temp conditions
        let conditions = self.filter().take_temp();
        self.filter().add_condition(Condition::Or(conditions));
        self
    }

    fn and_eq<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.and_eq_ref(c, v);
        self
    }

    fn and_eq_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.filter().add_condition(Condition::AndEq(c));
        self
    }

    fn or_eq<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.or_eq_ref(c, v);
        self
    }

    fn or_eq_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.filter().add_condition(Condition::OrEq(c));
        self
    }

}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::sql::select_statement::SelectStatement;
    use crate::sqlx::Sqlite;

    use super::*;

    struct Select<'a, DB> {
        _a: PhantomData<&'a DB>,
        statement: SelectStatement<'a>
    }

    impl<'a> Filter<'a, Sqlite> for Select<'a, Sqlite> {
        fn add_value<V>(&mut self, v: V) where V: Encode<'a, Sqlite> + Type<Sqlite> + Send + 'a {

        }

        fn filter(&mut self) -> &mut FilterStatement<'a> {
            &mut self.statement.filter
        }
    }

    #[test]
    fn test() {
        let s = Select { _a: Default::default(), statement: SelectStatement::from("user") };
        let id = 1;
        let name = "the user name";
        let v = s
            .and(|s| s.and_eq_ref("id", id).and_eq_ref("name", name))
            .or_eq("age", 15);


    }
}
