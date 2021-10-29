use sqlx::{ColumnIndex, Type};
use sqlx::decode::Decode;

use crate::Result;

macro_rules! gen_row {
    ($db: ty, $row: ty) => {
        pub struct Row {
            pub(crate) inner: $row
        }

        impl Row {
            pub fn try_get<'a, T, C>(&'a self, column: C) -> Result<T>
                where T: Decode<'a, $db> + Type<$db>, C: ColumnIndex<$row> {
                Ok(sqlx::Row::try_get(&self.inner, column)?)
            }
        }
    }
}

#[cfg(feature = "mysql")]
gen_row!(sqlx::MySql, sqlx::mysql::MySqlRow);
#[cfg(feature = "postgres")]
gen_row!(sqlx::Postgres, sqlx::postgres::PgRow);
#[cfg(feature = "sqlite")]
gen_row!(sqlx::Sqlite, sqlx::sqlite::SqliteRow);
#[cfg(feature = "mssql")]
gen_row!(sqlx::Mssql, sqlx::mssql::MssqlRow);
