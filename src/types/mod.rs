pub mod query_result;

pub use query_result::QueryResult;

pub(crate) type Result<T> = std::result::Result<T, anyhow::Error>;

macro_rules! gen_types {
    ($db: ty, $arg: ty, $row: ty) => {
        pub type Database = $db;
        pub(crate) type Pool = sqlx::pool::Pool<$db>;
        pub type Transaction<'a> = sqlx::Transaction<'a, $db>;
        pub type Arguments<'a> = $arg;
        pub type Row = $row;
    };
}

#[cfg(feature = "mysql")]
gen_types!(sqlx::MySql, sqlx::mysql::MySqlArguments, sqlx::mysql::MySqlRow);
#[cfg(feature = "postgres")]
gen_types!(sqlx::Postgres, sqlx::postgres::PgArguments, sqlx::postgres::PgRow);
#[cfg(feature = "sqlite")]
gen_types!(sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>, sqlx::sqlite::SqliteRow);
#[cfg(feature = "mssql")]
gen_types!(sqlx::Mssql, sqlx::mssql::MssqlArguments, sqlx::mssql::MssqlRow);
