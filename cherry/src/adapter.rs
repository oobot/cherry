pub enum AnyArguments<'a> {
    #[cfg(feature = "postgres")]
    Postgres(
        crate::sqlx::postgres::PgArguments,
        std::marker::PhantomData<&'a ()>,
    ),

    #[cfg(feature = "mysql")]
    MySql(
        crate::sqlx::mysql::MySqlArguments,
        std::marker::PhantomData<&'a ()>,
    ),

    #[cfg(feature = "sqlite")]
    Sqlite(crate::sqlx::sqlite::SqliteArguments<'a>),

    #[cfg(feature = "mssql")]
    Mssql(
        crate::sqlx::mssql::MssqlArguments,
        std::marker::PhantomData<&'a ()>,
    ),
}

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