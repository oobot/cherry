use sqlx::Database;

use crate::query_builder::where_clause::condition::Condition;
use crate::query_internal::r#where::Where;

pub trait WhereColumn<'a, DB>: Where<'a, DB> + Sized where DB: Database {

    fn and_eq_column(mut self, c: &'a str) -> Self {
        self.and_eq_column_ref(c);
        self
    }

    fn and_eq_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::AndEqColumn(c));
        self
    }

    fn or_eq_column(mut self, c: &'a str) -> Self {
        self.or_eq_column_ref(c);
        self
    }

    fn or_eq_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.add_where(Condition::OrEqColumn(c));
        self
    }

}