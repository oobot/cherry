use log::debug;
use sqlx::{Arguments as SqlxArguments, Type};
use sqlx::encode::Encode;

use crate::{Cherry, connection};
use crate::types::{Arguments, Database, Result};

pub struct RawSql<'a> {
    ds: &'a str,
    sql: String,
    arguments: Arguments<'a>,
}

impl<'a> RawSql<'a> {
    pub(crate) fn new(ds: &'a str) -> Self {
        Self { ds, sql: String::new(), arguments: Arguments::default() }
    }

    pub fn sql(mut self, sql: &str) ->  Self {
        self.sql = sql.to_string();
        self
    }

    pub fn bind<V>(mut self, v: V) -> Self
        where V: Encode<'a, Database> + Type<Database> + Send + 'a {
        self.arguments.add(v);
        self
    }

    pub async fn fetch<T: Cherry>(self) -> Result<Option<T>> {
        debug!("{}", self.sql);
        let row = sqlx::query_with(&self.sql, self.arguments)
            .fetch_optional(connection::get(self.ds)?).await?;
        match row {
            Some(row) => Ok(Some(T::from_row(&row)?)),
            _ => Ok(None)
        }
    }

    pub async fn fetch_all<T: Cherry>(self) -> Result<Vec<T>> {
        debug!("{}", self.sql);
        let rows = sqlx::query_with(&self.sql, self.arguments)
            .fetch_all(connection::get(self.ds)?).await?;
        let mut vec = Vec::with_capacity(rows.len());
        for row in rows {
            vec.push(T::from_row(&row)?);
        }
        Ok(vec)
    }

}
