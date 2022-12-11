use std::marker::PhantomData;

use anyhow::Error;
use sqlx::{Database, Encode, Executor, IntoArguments, Type};

use crate::arguments::Arguments;
use crate::Cherry;
use crate::database::AboutDatabase;

pub struct Select<'a, C, DB, ARGS> {
    arguments: ARGS,
    sql: String,
    _a: PhantomData<C>,
    _b: PhantomData<&'a DB>,
}

impl<'a, C, DB, ARGS> Select<'a, C, DB, ARGS>
    where C: Cherry<DB>,
          DB: Database + AboutDatabase<'a, DB, ARGS>,
          ARGS: Arguments<'a, DB> + IntoArguments<'a, DB> + Send +'a {

    pub fn new() -> Self {
        Self {
            arguments: DB::arguments(),
            sql: String::new(),
            _a: Default::default(),
            _b: Default::default(),
        }
    }

    pub fn by_id<T>(mut self, v: T) -> Self where T: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
        self
    }

    pub async fn one<'e, 'c: 'e, E>(mut self, e: E) -> Result<Option<C>, Error>
        where 'a: 'e,
              ARGS: 'e,
              E: Executor<'c, Database = DB> {
        // let table = <C as Cherry<DB>>::table();
        // self.sql = format!("select * from {} where id = ?", table);
        let row = sqlx::query_with("select * from user where id = ?", self.arguments)
            .fetch_optional(e).await?;
        let c = match row {
            Some(row) => Some(C::from_row(&row)?),
            _ => None,
        };
        Ok(c)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}