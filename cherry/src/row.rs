use sqlx::{ColumnIndex, Type};
use sqlx::mssql::MssqlRow;
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;

use crate::Result;

pub enum Row {
    MySqlRow(MySqlRow),
    PgRow(PgRow),
    SqliteRow(SqliteRow),
    MssqlRow(MssqlRow),
}

pub fn try_get<'a, R, T, C>(row: &'a R, column: C) -> Result<T>
    where R: sqlx::Row,
          T: sqlx::Decode<'a, R::Database> + Type<R::Database>,
          C: ColumnIndex<R> {
    Ok(row.try_get(column)?)
}


/*
pub fn decode_mysql<'a, T>(r: &'a MySqlRow, column: &str) -> Result<T>
    where T: sqlx::Decode<'a, <MySqlRow as Row>::Database> + Type<<MySqlRow as Row>::Database> {
    Ok(r.try_get(column)?)
}
pub fn decode_postgres<'a, T>(r: &'a PgRow, column: &str) -> Result<T>
    where T: sqlx::Decode<'a, <PgRow as Row>::Database> + Type<<PgRow as Row>::Database> {
    Ok(r.try_get(column)?)
}
pub fn decode_sqlite<'a, T>(r: &'a SqliteRow, column: &str) -> Result<T>
    where T: sqlx::Decode<'a, <SqliteRow as Row>::Database> + Type<<SqliteRow as Row>::Database> {
    Ok(r.try_get(column)?)
}
pub fn decode_mssql<'a, T>(r: &'a MssqlRow, column: &str) -> Result<T>
    where T: sqlx::Decode<'a, <MssqlRow as Row>::Database> + Type<<MssqlRow as Row>::Database> {
    Ok(r.try_get(column)?)
}
*/

/*
static ERR_MSG: &str = "Database row type mismatch.";

impl WrapRows {

    pub fn decode_mysql<'a, T>(&'a self, column: &str) -> Result<T>
        where T: sqlx::Decode<'a, MySql> + Type<MySql> {
        match self {
            WrapRows::MySqlRow(r) => Ok(r.try_get(column)?),
            _ => Err(cherry!(ERR_MSG))
        }
    }

    pub fn decode_postgres<'a, T>(&'a self, column: &str) -> Result<T>
        where T: sqlx::Decode<'a, Postgres> + Type<Postgres> {
        match self {
            WrapRows::PgRow(r) => Ok(r.try_get(column)?),
            _ => Err(cherry!(ERR_MSG))
        }
    }

    pub fn decode_sqlite<'a, T>(&'a self, column: &str) -> Result<T>
        where T: sqlx::Decode<'a, Sqlite> + Type<Sqlite> {
        match self {
            WrapRows::SqliteRow(r) => Ok(r.try_get(column)?),
            _ => Err(cherry!(ERR_MSG))
        }
    }

    pub fn decode_mssql<'a, T>(&'a self, column: &str) -> Result<T>
        where T: sqlx::Decode<'a, Mssql> + Type<Mssql> {
        match self {
            WrapRows::MssqlRow(r) => Ok(r.try_get(column)?),
            _ => Err(cherry!(ERR_MSG))
        }
    }

    // pub fn decode_any<'a, T>(&'a self, column: &str) -> Result<T>
    //     where T: sqlx::Decode<'a, Any> + Type<Any> {
    //     match self {
    //         WrapRows::AnyRow(r) => Ok(r.try_get(column)?),
    //         _ => Err(cherry!(ERR_MSG))
    //     }
    // }
}
*/