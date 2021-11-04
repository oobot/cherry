
pub(crate) type Result<T> = std::result::Result<T, anyhow::Error>;

macro_rules! gen_types {
    ($db: ty, $arg: ty, $row: ty, $qr: ty) => {
        pub type Database = $db;
        pub(crate) type Pool = sqlx::pool::Pool<$db>;
        pub type Transaction<'a> = sqlx::Transaction<'a, $db>;
        pub type Arguments<'a> = $arg;
        pub type Row = $row;
        pub type QueryResult = $qr;
    };
}

#[cfg(feature = "mysql")]
gen_types!(sqlx::MySql, sqlx::mysql::MySqlArguments, sqlx::mysql::MySqlRow, sqlx::mysql::MySqlQueryResult);
#[cfg(feature = "postgres")]
gen_types!(sqlx::Postgres, sqlx::postgres::PgArguments, sqlx::postgres::PgRow, sqlx::postgres::PgQueryResult);
#[cfg(feature = "sqlite")]
gen_types!(sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>, sqlx::sqlite::SqliteRow, sqlx::sqlite::SqliteQueryResult);
#[cfg(feature = "mssql")]
gen_types!(sqlx::Mssql, sqlx::mssql::MssqlArguments, sqlx::mssql::MssqlRow, sqlx::mssql::MssqlQueryResult);
