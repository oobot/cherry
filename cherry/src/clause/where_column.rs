use sqlx::Database;

use crate::clause::where_value::Where;
use crate::sql::where_condition::Condition;

pub trait WhereColumn<'a, DB>: Where<'a, DB> + Sized where DB: Database {

    fn and_eq_column(mut self, c: &'a str) -> Self {
        self.and_eq_column_ref(c);
        self
    }

    fn and_eq_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::AndEqColumn(c));
        self
    }

    fn or_eq_column(mut self, c: &'a str) -> Self {
        self.or_eq_column_ref(c);
        self
    }

    fn or_eq_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::OrEqColumn(c));
        self
    }

    fn and_ge_column(mut self, c: &'a str) -> Self {
        self.and_ge_column_ref(c);
        self
    }

    fn and_ge_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::AndGeColumn(c));
        self
    }

    fn or_ge_column(mut self, c: &'a str) -> Self {
        self.or_ge_column_ref(c);
        self
    }

    fn or_ge_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::OrGeColumn(c));
        self
    }

    fn and_gt_column(mut self, c: &'a str) -> Self {
        self.and_gt_column_ref(c);
        self
    }

    fn and_gt_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::AndGtColumn(c));
        self
    }

    fn or_gt_column(mut self, c: &'a str) -> Self {
        self.or_gt_column_ref(c);
        self
    }

    fn or_gt_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::OrGtColumn(c));
        self
    }

    fn and_le_column(mut self, c: &'a str) -> Self {
        self.and_le_column_ref(c);
        self
    }

    fn and_le_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::AndLeColumn(c));
        self
    }

    fn or_le_column(mut self, c: &'a str) -> Self {
        self.or_le_column_ref(c);
        self
    }

    fn or_le_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::OrLeColumn(c));
        self
    }

    fn and_lt_column(mut self, c: &'a str) -> Self {
        self.and_lt_column_ref(c);
        self
    }

    fn and_lt_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::AndLtColumn(c));
        self
    }

    fn or_lt_column(mut self, c: &'a str) -> Self {
        self.or_lt_column_ref(c);
        self
    }

    fn or_lt_column_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::OrLtColumn(c));
        self
    }

    fn and_column_is_null(mut self, c: &'a str) -> Self {
        self.and_column_is_null_ref(c);
        self
    }

    fn and_column_is_null_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::AndColumnIsNull(c));
        self
    }

    fn or_column_is_null(mut self, c: &'a str) -> Self {
        self.or_column_is_null_ref(c);
        self
    }

    fn or_column_is_null_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::OrColumnIsNull(c));
        self
    }


    fn and_column_is_not_null(mut self, c: &'a str) -> Self {
        self.and_column_is_not_null_ref(c);
        self
    }

    fn and_column_is_not_null_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::AndColumnIsNotNull(c));
        self
    }

    fn or_column_is_not_null(mut self, c: &'a str) -> Self {
        self.or_column_is_not_null_ref(c);
        self
    }

    fn or_column_is_not_null_ref(&mut self, c: &'a str) -> &mut Self {
        self.sql_builder().add_where(Condition::OrColumnIsNotNull(c));
        self
    }

}