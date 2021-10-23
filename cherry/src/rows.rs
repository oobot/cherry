use sqlx::{Any, Mssql, MySql, Postgres, Row, Sqlite, Type};
use sqlx::any::AnyRow;
use sqlx::mssql::MssqlRow;
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;

static ERR_MSG: &str = "Database row type mismatch.";

pub enum WrapRows {
    MySqlRow(MySqlRow),
    PgRow(PgRow),
    SqliteRow(SqliteRow),
    MssqlRow(MssqlRow),
    AnyRow(AnyRow),
}

impl WrapRows {

    pub fn decode_mysql<'a, T>(&'a self, column: &str) -> Result<T, anyhow::Error>
        where T: sqlx::Decode<'a, MySql> + Type<MySql> {
        match self {
            WrapRows::MySqlRow(r) => Ok(r.try_get(column)?),
            _ => Err(anyhow!(ERR_MSG))
        }
    }

    pub fn decode_postgres<'a, T>(&'a self, column: &str) -> Result<T, anyhow::Error>
        where T: sqlx::Decode<'a, Postgres> + Type<Postgres> {
        match self {
            WrapRows::PgRow(r) => Ok(r.try_get(column)?),
            _ => Err(anyhow!(ERR_MSG))
        }
    }

    pub fn decode_sqlite<'a, T>(&'a self, column: &str) -> Result<T, anyhow::Error>
        where T: sqlx::Decode<'a, Sqlite> + Type<Sqlite> {
        match self {
            WrapRows::SqliteRow(r) => Ok(r.try_get(column)?),
            _ => Err(anyhow!(ERR_MSG))
        }
    }

    pub fn decode_mssql<'a, T>(&'a self, column: &str) -> Result<T, anyhow::Error>
        where T: sqlx::Decode<'a, Mssql> + Type<Mssql> {
        match self {
            WrapRows::MssqlRow(r) => Ok(r.try_get(column)?),
            _ => Err(anyhow!(ERR_MSG))
        }
    }

    pub fn decode_any<'a, T>(&'a self, column: &str) -> Result<T, anyhow::Error>
        where T: sqlx::Decode<'a, Any> + Type<Any> {
        match self {
            WrapRows::AnyRow(r) => Ok(r.try_get(column)?),
            _ => Err(anyhow!(ERR_MSG))
        }
    }
}
