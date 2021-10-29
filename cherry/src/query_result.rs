macro_rules! from_query_result {
    () => {
        #[cfg(feature = "mysql")]
        pub(crate) fn from(result: sqlx::mysql::MySqlQueryResult) -> Self {
            Self {
                rows_affected: result.rows_affected(),
                last_insert_id: result.last_insert_id()
            }
        }

        #[cfg(feature = "postgres")]
        pub(crate) fn from(result: sqlx::postgres::PgQueryResult) -> Self {
            Self {
                rows_affected: result.rows_affected(),
            }
        }

        #[cfg(feature = "sqlite")]
        pub(crate) fn from(result: sqlx::sqlite::SqliteQueryResult) -> Self {
            Self {
                rows_affected: result.rows_affected(),
                last_insert_rowid: result.last_insert_rowid(),
            }
        }

        #[cfg(feature = "mssql")]
        pub(crate) fn from(result: sqlx::mssql::MssqlQueryResult) -> Self {
            Self {
                rows_affected: result.rows_affected(),
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct QueryResult {
    pub(crate) rows_affected: u64,

    #[cfg(feature = "mysql")]
    pub(crate) last_insert_id: u64,

    #[cfg(feature = "sqlite")]
    pub(crate) last_insert_rowid: i64,
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

    from_query_result!();
}
