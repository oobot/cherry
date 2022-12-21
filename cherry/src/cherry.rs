use crate::query::Query;

pub trait Cherry<'a, DB>: Sized + Send + Unpin
    where
        DB: sqlx::Database, {

    fn table() -> &'static str;

    // field name -> column name
    fn columns() -> Vec<(&'static str, &'static str)>;

    fn arguments(&'a self, arguments: &mut <DB as sqlx::database::HasArguments<'a>>::Arguments);

    fn from_row(row: &<DB as sqlx::Database>::Row) -> Result<Self, crate::Error>;

    fn insert(&'a self) -> Query<'a, Self, DB> {
        Query::new_insert(self)
    }

    fn insert_bulk(v: &'a [Self]) -> Query<'a, Self, DB> {
        Query::new_insert_bulk(v)
    }

    fn update() -> Query<'a, Self, DB> {
        Query::new_update()
    }

    fn select() -> Query<'a, Self, DB> {
        Query::new_select()
    }

    fn delete() -> Query<'a, Self, DB> {
        Query::new_delete()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::Cherry;
    use crate::sqlx::{Database, Sqlite};
    use crate::sqlx::database::HasArguments;

    struct Example {
        id: u32,
    }

    impl<'a> Cherry<'a, crate::sqlx::Sqlite> for Example {
        fn table() -> &'static str {
            todo!()
        }

        fn columns() -> Vec<(&'static str, &'static str)> {
            todo!()
        }

        fn arguments(&'a self, arguments: &mut <Sqlite as HasArguments<'a>>::Arguments) {
            use sqlx::Arguments;
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