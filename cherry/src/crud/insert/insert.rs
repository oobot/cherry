use std::marker::PhantomData;
use sqlx::{Database, IntoArguments};
use crate::arguments::Arguments;
use crate::Cherry;
use crate::database::AboutDatabase;

pub struct Insert<'a, C, DB, ARGS> {
    pub(crate) v: &'a C,
    _a: PhantomData<DB>,
    _b: PhantomData<ARGS>,
}

impl<'a, C, DB, ARGS> Insert<'a, C, DB, ARGS>
    where C: Cherry<DB>,
          DB: Database + AboutDatabase<'a, DB, ARGS>,
          ARGS: Arguments<'a, DB> + IntoArguments<'a, DB> + Send +'a {

    pub fn from(v: &'a C) -> Self {
        Self { v, _a: Default::default(), _b: Default::default() }
    }

}