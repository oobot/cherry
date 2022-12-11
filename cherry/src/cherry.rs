use crate::adapter::{AnyArguments, AnyRow};

pub trait Cherry<DB: crate::sqlx::Database>: Sized + Send + Unpin {

    fn table() -> &'static str;

    // field name -> column name
    fn columns() -> Vec<(&'static str, &'static str)>;

    // fn arguments<'a>(&'a self, arguments: &mut AnyArguments<'a>);

    fn from_row(row: &<DB as sqlx::Database>::Row) -> Result<Self, crate::Error>;

    // CRUD
}

#[cfg(test)]
mod tests {
    use anyhow::Error;
    use crate::adapter::{AnyArguments, AnyRow};
    use crate::Cherry;
    use crate::sqlx::{Database, Sqlite};

    struct Example {
        id: u32,
    }

    impl Cherry<crate::sqlx::Sqlite> for Example {
        fn table() -> &'static str {
            todo!()
        }

        fn columns() -> Vec<(&'static str, &'static str)> {
            todo!()
        }

        // fn arguments<'a>(&'a self, arguments: &mut AnyArguments<'a>) {
        //     match arguments {
        //         #[cfg(feature = "postgres")]
        //         AnyArguments::Postgres(arguments, _) => { arguments;}
        //         #[cfg(feature = "mysql")]
        //         AnyArguments::MySql(arguments, _) => { arguments; }
        //         #[cfg(feature = "sqlite")]
        //         AnyArguments::Sqlite(arguments) => { arguments; }
        //         #[cfg(feature = "mssql")]
        //         AnyArguments::Mssql(arguments, _) => { arguments; }
        //     }
        // }

        fn from_row(row: &<Sqlite as Database>::Row) -> Result<Self, Error> {
            todo!()
        }
    }

    #[test]
    fn unit_test() {

    }

}