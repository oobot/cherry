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

macro_rules! gen_query_result {
    ($qr: ty) => {
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
gen_query_result!(sqlx::mysql::MySqlQueryResult);
#[cfg(feature = "postgres")]
gen_query_result!(sqlx::postgres::PgQueryResult);
#[cfg(feature = "sqlite")]
gen_query_result!(sqlx::sqlite::SqliteQueryResult);
#[cfg(feature = "mssql")]
gen_query_result!(sqlx::mssql::MssqlQueryResult);