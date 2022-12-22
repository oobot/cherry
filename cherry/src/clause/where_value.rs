use sqlx::{Database, Encode, Type};

use crate::provider::WhereProvider;
use crate::sql::where_clause::condition::Condition;

pub trait Where<'a, DB>: WhereProvider<'a, DB> + Sized where DB: Database {

    // wrap conditions
    fn and<F>(mut self, mut f: F) -> Self where F: FnMut(&mut Self) -> &mut Self {
        self.surround_where();
        f(&mut self); // closure will add to temp conditions
        let conditions = self.take_surround();
        self.add_where(Condition::And(conditions));
        self
    }

    // wrap conditions
    fn or<F>(mut self, mut f: F) -> Self where F: FnMut(&mut Self) -> &mut Self {
        self.surround_where();
        f(&mut self); // closure will add to temp conditions
        let conditions = self.take_surround();
        self.add_where(Condition::Or(conditions));
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
        self.add_where(Condition::AndEq(c));
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
        self.add_where(Condition::OrEq(c));
        self
    }

    fn and_ge<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.and_ge_ref(c, v);
        self
    }

    fn and_ge_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_where(Condition::AndGe(c));
        self
    }

    fn or_ge<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.or_ge_ref(c, v);
        self
    }

    fn or_ge_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_where(Condition::OrGe(c));
        self
    }

    fn and_gt<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.and_gt_ref(c, v);
        self
    }

    fn and_gt_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_where(Condition::AndGt(c));
        self
    }

    fn or_gt<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.or_gt_ref(c, v);
        self
    }

    fn or_gt_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_where(Condition::OrGt(c));
        self
    }


    fn and_le<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.and_le_ref(c, v);
        self
    }

    fn and_le_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_where(Condition::AndLe(c));
        self
    }

    fn or_le<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.or_le_ref(c, v);
        self
    }

    fn or_le_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_where(Condition::OrLe(c));
        self
    }

    fn and_lt<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.and_lt_ref(c, v);
        self
    }

    fn and_lt_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_where(Condition::AndLt(c));
        self
    }

    fn or_lt<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.or_lt_ref(c, v);
        self
    }

    fn or_lt_ref<V>(&mut self, c: &'a str, v: V) -> &mut Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_where(Condition::OrLt(c));
        self
    }

    fn and_is_null(mut self, c: &'a str) -> Self {
        self.and_is_null_ref(c);
        self
    }

    fn and_is_null_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::AndIsNull(c));
        self
    }

    fn or_is_null(mut self, c: &'a str) -> Self {
        self.or_is_null_ref(c);
        self
    }

    fn or_is_null_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::OrIsNull(c));
        self
    }

    fn and_is_not_null(mut self, c: &'a str) -> Self {
        self.and_is_not_null_ref(c);
        self
    }

    fn and_is_not_null_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::AndIsNotNull(c));
        self
    }

    fn or_is_not_null(mut self, c: &'a str) -> Self {
        self.or_is_not_null_ref(c);
        self
    }

    fn or_is_not_null_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::OrIsNotNull(c));
        self
    }

    fn and_between(mut self, c: &'a str) -> Self {
        self.and_between_ref(c);
        self
    }

    fn and_between_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::AndBetween(c));
        self
    }

    fn or_between(mut self, c: &'a str) -> Self {
        self.or_between_ref(c);
        self
    }

    fn or_between_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::OrBetween(c));
        self
    }

    fn and_not_between(mut self, c: &'a str) -> Self {
        self.and_not_between_ref(c);
        self
    }

    fn and_not_between_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::AndNotBetween(c));
        self
    }

    fn or_not_between(mut self, c: &'a str) -> Self {
        self.or_not_between_ref(c);
        self
    }

    fn or_not_between_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::OrNotBetween(c));
        self
    }

    fn and_in<V, I>(mut self, c: &'a str, v: I) -> Self
        where
            V: Encode<'a, DB> + Type<DB> + Send + 'a,
            I: IntoIterator<Item = V> {
        self.and_in_ref(c, v);
        self
    }

    fn and_in_ref<V, I>(&mut self, c: &'a str, v: I) -> &mut Self
        where
            V: Encode<'a, DB> + Type<DB> + Send + 'a,
            I: IntoIterator<Item = V> {
        let length = v.into_iter().map(|v| self.add_value(v)).count();
        self.add_where(Condition::AndIn(c, length));
        self
    }

    fn or_in<V, I>(mut self, c: &'a str, v: I) -> Self
        where
            V: Encode<'a, DB> + Type<DB> + Send + 'a,
            I: IntoIterator<Item = V> {
        self.or_in_ref(c, v);
        self
    }

    fn or_in_ref<V, I>(&mut self, c: &'a str, v: I) -> &mut Self
        where
            V: Encode<'a, DB> + Type<DB> + Send + 'a,
            I: IntoIterator<Item = V> {
        let length = v.into_iter().map(|v| self.add_value(v)).count();
        self.add_where(Condition::OrIn(c, length));
        self
    }

    fn and_not_in<V, I>(mut self, c: &'a str, v: I) -> Self
        where
            V: Encode<'a, DB> + Type<DB> + Send + 'a,
            I: IntoIterator<Item = V> {
        self.and_not_in_ref(c, v);
        self
    }

    fn and_not_in_ref<V, I>(&mut self, c: &'a str, v: I) -> &mut Self
        where
            V: Encode<'a, DB> + Type<DB> + Send + 'a,
            I: IntoIterator<Item = V> {
        let length = v.into_iter().map(|v| self.add_value(v)).count();
        self.add_where(Condition::AndNotIn(c, length));
        self
    }

    fn or_not_in<V, I>(mut self, c: &'a str, v: I) -> Self
        where
            V: Encode<'a, DB> + Type<DB> + Send + 'a,
            I: IntoIterator<Item = V> {
        self.or_not_in_ref(c, v);
        self
    }

    fn or_not_in_ref<V, I>(&mut self, c: &'a str, v: I) -> &mut Self
        where
            V: Encode<'a, DB> + Type<DB> + Send + 'a,
            I: IntoIterator<Item = V> {
        let length = v.into_iter().map(|v| self.add_value(v)).count();
        self.add_where(Condition::OrNotIn(c, length));
        self
    }


}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::sqlx::Sqlite;

    use super::*;

    #[test]
    fn test() {
        // let s = Select { _a: Default::default(), statement: SelectStatement::from("user") };
        // let id = 1;
        // let name = "the user name";
        // let v = s
        //     .and(|s| s.and_eq_ref("id", id).and_eq_ref("name", name))
        //     .or_eq("age", 15);

    }
}
