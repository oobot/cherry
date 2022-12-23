use sqlx::Database;

use crate::provider::Provider;
use crate::sql::select_column::Column;

pub trait SelectColumn<'a, DB>: Provider<'a, DB> + Sized where DB: Database {

    fn column(mut self, c: &'a str) -> Self {
        self.sql_builder().add_select_column(Column::Column(c));
        self
    }

    fn column_raw(mut self, raw: &'a str) -> Self {
        self.sql_builder().add_select_column(Column::Raw(raw));
        self
    }

}