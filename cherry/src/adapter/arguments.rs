pub enum AnyArguments<'a> {
    #[cfg(feature = "postgres")]
    Postgres(
        sqlx::postgres::PgArguments,
        std::marker::PhantomData<&'a ()>,
    ),

    #[cfg(feature = "mysql")]
    MySql(
        sqlx::mysql::MySqlArguments,
        std::marker::PhantomData<&'a ()>,
    ),

    #[cfg(feature = "sqlite")]
    Sqlite(sqlx::sqlite::SqliteArguments<'a>),

    #[cfg(feature = "mssql")]
    Mssql(
        sqlx::mssql::MssqlArguments,
        std::marker::PhantomData<&'a ()>,
    ),
}