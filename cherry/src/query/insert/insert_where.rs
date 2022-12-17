use sqlx::Database;

use crate::query::r#where::Where;
use crate::query_builder::where_clause::condition::Condition;

pub trait InsertWhere<'a, DB>: Where<'a, DB> + Sized where DB: Database {

    fn and_eq_column(mut self, c: &'a str) -> Self {
        self.and_eq_column_ref(c);
        self
    }

    fn and_eq_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where_condition(Condition::AndEqColumn(c));
        self
    }

    fn or_eq_column(mut self, c: &'a str) -> Self {
        self.or_eq_column_ref(c);
        self
    }

    fn or_eq_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where_condition(Condition::OrEqColumn(c));
        self
    }

}