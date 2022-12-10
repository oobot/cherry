pub enum AnyRow {
    #[cfg(feature = "postgres")]
    Postgres(crate::sqlx::postgres::PgRow),

    #[cfg(feature = "mysql")]
    MySql(crate::sqlx::mysql::MySqlRow),

    #[cfg(feature = "sqlite")]
    Sqlite(crate::sqlx::sqlite::SqliteRow),

    #[cfg(feature = "mssql")]
    Mssql(crate::sqlx::mssql::MssqlRow),
}