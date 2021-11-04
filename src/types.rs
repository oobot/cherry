pub(crate) type Result<T> = std::result::Result<T, anyhow::Error>;

pub struct QueryResult {
    rows_affected: u64,
    #[cfg(any(feature = "mysql"))]
    last_insert_id: u64,
    #[cfg(feature = "sqlite")]
    last_insert_rowid: i64,
}

impl QueryResult {
    pub fn rows_affected(&self) -> u64 {
        self.rows_affected
    }

    #[cfg(feature = "mysql")]
    pub fn last_insert_id(&self) -> u64 {
        self.last_insert_id
    }

    #[cfg(feature = "sqlite")]
    pub fn last_insert_rowid(&self) -> i64 {
        self.last_insert_rowid
    }
}

macro_rules! gen_types {
    ($db: ty, $arg: ty, $row: ty, $qr: ty) => {
        pub type Database = $db;
        pub(crate) type Pool = sqlx::pool::Pool<$db>;
        pub type Transaction<'a> = sqlx::Transaction<'a, $db>;
        pub type Arguments<'a> = $arg;
        pub type Row = $row;

        impl QueryResult {
            pub(crate) fn from(result: $qr) -> Self {
                Self {
                    rows_affected: result.rows_affected(),
                    #[cfg(feature = "mysql")]
                    last_insert_id: result.last_insert_id(),
                    #[cfg(feature = "sqlite")]
                    last_insert_rowid: result.last_insert_rowid(),
                }
            }
        }
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
