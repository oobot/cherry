use std::marker::PhantomData;

use sqlx::{Database, Encode, Type};

use crate::arguments::Arguments;
use crate::Cherry;
use crate::database::AboutDatabase;

pub struct Select<'a, DB, ARGS> {
    arguments: ARGS,
    _marker: PhantomData<&'a DB>,
}

impl<'a, DB, ARGS> Select<'a, DB, ARGS>
    where DB: Database + AboutDatabase<'a, DB, ARGS>,
          ARGS: Send + Arguments<'a, DB> {

    pub fn new() -> Self {
        Self {
            arguments: DB::arguments(),
            _marker: Default::default(),
        }
    }

    pub fn add<T>(&mut self, v: T) where T: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}