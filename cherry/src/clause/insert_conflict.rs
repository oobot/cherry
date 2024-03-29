use sqlx::Database;

use crate::provider::Provider;
use crate::sql::InsertOnConflict;

pub trait InsertConflict<'a, DB>: Provider<'a, DB> + Sized where DB: Database {

    fn ignore_on_conflict(mut self) -> Self {
        self.sql_builder().conflict_with(InsertOnConflict::Ignore);
        self
    }
    
    fn update_on_conflict(mut self) -> Self {
        self.sql_builder().conflict_with(InsertOnConflict::Update);
        self
    }
    
    #[cfg(any(feature = "sqlite", feature = "mysql"))]
    fn replace_on_conflict(mut self) -> Self {
        self.sql_builder().conflict_with(InsertOnConflict::Replace);
        self
    }
    
    #[cfg(any(feature = "sqlite", feature = "postgres"))]
    fn conflict_column(mut self, column: &'a str) -> Self {
        self.sql_builder().add_conflict_column(column);
        self
    }
    
    #[cfg(any(feature = "sqlite", feature = "postgres"))]
    fn conflict_columns<I>(mut self, columns: I) -> Self
        where
            I: IntoIterator<Item = &'a str> {
        columns.into_iter().for_each(|column|
             self.sql_builder().add_conflict_column(column)
        );
        self
    }

}