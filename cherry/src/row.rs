use sqlx::{ColumnIndex, Database, Type};
use sqlx::decode::Decode;

use crate::Result;

pub struct Row<DB: Database> {
    pub(crate) inner: DB::Row
}

impl<DB: Database> Row<DB> {
    pub fn decode<'a, T, C>(&'a self, column: C) -> Result<T>
        where T: Decode<'a, DB> + Type<DB>, C: ColumnIndex<DB::Row> {
        Ok(sqlx::Row::try_get(&self.inner, column)?)
    }
}
