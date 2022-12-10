use sqlx::{Arguments, IntoArguments};
use sqlx::database::HasArguments;

pub enum Database {
    #[cfg(feature = "postgres")]
    Postgres(sqlx::Postgres),

    #[cfg(feature = "mysql")]
    MySql(sqlx::MySql),

    #[cfg(feature = "sqlite")]
    Sqlite(sqlx::Sqlite),
}

impl Database {

    /*pub(crate) fn new_arguments<'a, A, DB>(&'a self)
        -> Box<dyn Arguments<'a, Database=DB>>
        where DB: sqlx::Database
    {
        /*match self {
            #[cfg(feature = "postgres")]
            Database::Postgres(a) => {
                use sqlx::Arguments;
                <a as HasArguments<'a>>::Arguments
                // crate::sqlx::postgres::PgArguments::default()
            }
            #[cfg(feature = "mysql")]
            Database::MySql(a) => {
                crate::sqlx::mysql::MySqlArguments::default()
            }
            #[cfg(feature = "sqlite")]
            Database::Sqlite(a) => {
                crate::sqlx::sqlite::SqliteArguments::default()
            }
            #[cfg(feature = "mssql")]
            Database::Mssql(_) => {
                todo!()
            }
        }*/
        todo!()
    }*/

}