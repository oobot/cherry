use std::marker::PhantomData;

use anyhow::Error;
use sqlx::{Database, Encode, Executor, IntoArguments, Type};

use crate::arguments::Arguments;
use crate::Cherry;
use crate::database::AboutDatabase;

pub struct Select<'a, C, DB, ARGS> {
    arguments: ARGS,
    _t: PhantomData<C>,
    _marker: PhantomData<&'a DB>,
}

impl<'a, C, DB, ARGS> Select<'a, C, DB, ARGS>
    where C: Cherry,
          DB: Database + AboutDatabase<'a, DB, ARGS>,
          ARGS: Arguments<'a, DB> + IntoArguments<'a, DB> + Send +'a {

    pub fn new() -> Self {
        Self {
            arguments: DB::arguments(),
            _t: Default::default(),
            _marker: Default::default(),
        }
    }

    pub fn by_id<T>(&mut self, v: T) where T: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    pub async fn one<'e, 'c: 'e, E>(mut self, e: E) -> Result<Option<C>, Error>
        where 'a: 'e,
              ARGS: 'e,
              E: Executor<'c, Database = DB> {
        let row = sqlx::query_with("", self.arguments)
            .fetch_one(e).await?;

        todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}