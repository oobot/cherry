
pub trait Cherry<'a, DB, A>: Sized + Send + Unpin
    where
        DB: crate::sqlx::Database,
        A: crate::arguments::Arguments<'a, DB> {

    fn table() -> &'static str;

    // field name -> column name
    fn columns() -> Vec<(&'static str, &'static str)>;

    fn arguments(&'a self, arguments: &mut A) {  }

    fn from_row(row: &<DB as sqlx::Database>::Row) -> Result<Self, crate::Error>;

    // CRUD
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::arguments::Arguments;
    use crate::arguments::sqlite::SqliteArguments;
    use crate::Cherry;
    use crate::sqlx::{Database, Sqlite};
    use crate::sqlx::database::HasArguments;

    struct Example {
        id: u32,
    }

    impl<'a> Cherry<'a, crate::sqlx::Sqlite, SqliteArguments<'a>> for Example {
        fn table() -> &'static str {
            todo!()
        }

        fn columns() -> Vec<(&'static str, &'static str)> {
            todo!()
        }

        fn arguments(&'a self, arguments: &mut SqliteArguments<'a>) {
            arguments.add(1);
        }

        fn from_row(row: &<Sqlite as Database>::Row) -> Result<Self, Error> {
            todo!()
        }
    }

    #[test]
    fn unit_test() {

    }

}