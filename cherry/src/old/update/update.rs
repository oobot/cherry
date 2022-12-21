use std::marker::PhantomData;

use anyhow::Error;
use sqlx::{Database, Encode, Executor, Type};

use crate::arguments::Arguments;
use crate::Cherry;
use crate::query_builder::set_clause::SetSection;
use crate::query_builder::TargetQuery;
use crate::query_builder::update::UpdateBuilder;
use crate::query_builder::where_clause::condition::Condition;
use crate::query_internal::provider::{SetProvider, WhereProvider};
use crate::query_internal::r#where::Where;
use crate::query_internal::set::UpdateSet;

pub struct Update<'a, T, DB, A> {
    arguments: A,
    sql: &'a mut String,
    query_builder: UpdateBuilder<'a>,
    _a: PhantomData<DB>,
    _b: PhantomData<T>,
}

impl<'a, T, DB, A> Update<'a, T, DB, A>
    where T: Cherry<'a, DB, A> + 'a,
          DB: Database,
          A: Arguments<'a, DB> + Send +'a {

    pub fn from(sql: &'a mut String) -> Self {
        assert!(sql.is_empty());
        Self {
            arguments: A::new(),
            sql,
            query_builder: UpdateBuilder::from(TargetQuery::new::<DB>(), T::table()),
            _a: Default::default(), _b: Default::default(),
        }
    }

    pub async fn execute<E>(self, e: E) -> Result<DB::QueryResult, Error>
        where E: Executor<'a, Database = DB> {
        self.sql.push_str(self.query_builder.as_sql().as_str());
        Ok(sqlx::query_with(self.sql, self.arguments).execute(e).await?)
    }

}

impl<'a, T, DB, A> SetProvider<'a, DB> for Update<'a, T, DB, A>
    where T: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn add_set_section(&mut self, section: SetSection<'a>) {
        self.query_builder.set_clause.add(section);
    }
}

impl<'a, T, DB, A> UpdateSet<'a, DB> for Update<'a, T, DB, A>
    where T: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}


impl<'a, T, DB, A> WhereProvider<'a, DB> for Update<'a, T, DB, A>
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

impl<'a, T, DB, A> Where<'a, DB> for Update<'a, T, DB, A>
    where T: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}
