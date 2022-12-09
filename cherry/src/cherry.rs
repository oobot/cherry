use crate::adapter::{AnyArguments, AnyRow};

pub trait Cherry: Sized + Send + Unpin {

    fn table() -> &'static str;

    // field name -> column name
    fn columns() -> Vec<(&'static str, &'static str)>;

    fn arguments<'a>(&'a self, arguments: &mut AnyArguments<'a>);

    fn from_row(row: &AnyRow) -> Result<Self, crate::Error>;

    // CRUD
}

#[cfg(test)]
mod tests {
    use crate::adapter::{AnyArguments, AnyRow};
    use crate::Cherry;

    struct Example {
        id: u32,
    }

    impl Cherry for Example {
        fn table() -> &'static str {
            todo!()
        }

        fn columns() -> Vec<(&'static str, &'static str)> {
            todo!()
        }

        fn arguments<'a>(&'a self, arguments: &mut AnyArguments<'a>) {
            match arguments {
                #[cfg(feature = "postgres")]
                AnyArguments::Postgres(arguments, _) => { arguments;}
                #[cfg(feature = "mysql")]
                AnyArguments::MySql(arguments, _) => { arguments; }
                #[cfg(feature = "sqlite")]
                AnyArguments::Sqlite(arguments) => { arguments; }
                #[cfg(feature = "mssql")]
                AnyArguments::Mssql(arguments, _) => { arguments; }
            }
        }

        fn from_row(row: &AnyRow) -> Result<Self, crate::Error> {
            // match row {
            //     #[cfg(feature = "postgres")]
            //     AnyRow::Postgres(row) => { }
            //     #[cfg(feature = "mysql")]
            //     AnyRow::MySql(row) => {}
            //     #[cfg(feature = "sqlite")]
            //     AnyRow::Sqlite(row) => {}
            //     #[cfg(feature = "mssql")]
            //     AnyRow::Mssql(row) => {}
            // }

            todo!()
        }
    }

    #[test]
    fn unit_test() {

    }

}