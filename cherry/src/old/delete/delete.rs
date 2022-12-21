use std::marker::PhantomData;

use anyhow::Error;
use sqlx::{Database, Encode, Executor, Type};

use crate::arguments::Arguments;
use crate::Cherry;
use crate::query_builder::delete::DeleteBuilder;
use crate::query_builder::TargetQuery;
use crate::query_builder::where_clause::condition::Condition;
use crate::query_internal::provider::WhereProvider;
use crate::query_internal::r#where::Where;

pub struct Delete<'a, T, DB, A> {
    arguments: A,
    query_builder: DeleteBuilder<'a>,
    _a: PhantomData<DB>,
    _b: PhantomData<T>,
}

impl<'a, T, DB, A> Delete<'a, T, DB, A>
    where T: Cherry<'a, DB, A> + 'a,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

    pub fn new() -> Self {
        // assert!(sql.is_empty());
        Self {
            arguments: A::new(),
            query_builder: DeleteBuilder::from(TargetQuery::new::<DB>(), T::table()),
            _a: Default::default(), _b: Default::default(),
        }
    }

    pub async fn execute<'e, E>(mut self, e: E) -> Result<DB::QueryResult, Error>
        where
            E: Executor<'e, Database =DB> {
        // Ok(sqlx::query_with(&self.query_builder.as_sql(), self.arguments).execute(e).await?)
        todo!()
    }

}

impl<'a, T, DB, A> WhereProvider<'a, DB> for Delete<'a, T, DB, A>
    where T: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn make_wrap(&mut self) {
        self.query_builder.where_clause.make_temp();
    }

    fn take_wrap(&mut self) -> Vec<Condition<'a>> {
        self.query_builder.where_clause.take_temp()
    }

    fn add_where_condition(&mut self, c: Condition<'a>) {
        self.query_builder.where_clause.add(c);
    }
}

impl<'a, T, DB, A> Where<'a, DB> for Delete<'a, T, DB, A>
    where T: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}
