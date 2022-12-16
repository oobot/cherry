use std::marker::PhantomData;

use sqlx::{Database, Encode, Executor, IntoArguments, Type};

use crate::{Cherry, Error};
use crate::arguments::Arguments;
use crate::database::AboutDatabase;
use crate::query::provider::{SetProvider, WhereProvider};
use crate::query::r#where::Where;
use crate::query::set::Set;
use crate::query_builder::insert::{Conflict, InsertBuilder};
use crate::query_builder::set_clause::SetSection;
use crate::query_builder::where_clause::condition::Condition;

pub struct Insert<'a, T, DB, A> {
    arguments: A,
    rows_count: usize,
    sql: &'a mut String,
    query_builder: InsertBuilder<'a>,
    _a: PhantomData<DB>,
    _b: PhantomData<T>,
}

impl<'a, T, DB, A> Insert<'a, T, DB, A>
    where T: Cherry<'a, DB, A> + 'a,
          DB: Database + AboutDatabase<'a, DB, A>,
          A: Arguments<'a, DB> + IntoArguments<'a, DB> + Send +'a {

    pub fn from_one(v: &'a T, sql: &'a mut String) -> Self {
        assert!(sql.is_empty());
        let mut arguments = DB::arguments();
        v.arguments(&mut arguments);
        Self {
            arguments, rows_count: 1, sql,
            query_builder: Self::create_query_builder(1),
            _a: Default::default(), _b: Default::default(),
        }
    }

    pub fn from_multiple(v: &'a [T], sql: &'a mut String) -> Self {
        assert!(sql.is_empty());
        let mut arguments = DB::arguments();
        v.iter().for_each(|row| row.arguments(&mut arguments));
        Self {
            arguments, rows_count: v.len(), sql,
            query_builder: Self::create_query_builder(v.len()),
            _a: Default::default(), _b: Default::default(),
        }
    }

    fn create_query_builder(rows_count: usize) -> InsertBuilder<'a> {
        InsertBuilder::from(
            DB::target(),
            T::table(),
            T::columns().into_iter().map(|(_f, c)| c).collect(),
            rows_count,
        )
    }

    pub fn on_conflict_ignore(mut self) -> Self {
        self.query_builder.conflict(Conflict::Ignore);
        self
    }

    pub fn on_conflict_update(mut self) -> Self {
        self.query_builder.conflict(Conflict::Update);
        self
    }

    #[cfg(any(feature = "sqlite", feature = "mysql"))]
    pub fn on_conflict_replace(mut self) -> Self {
        self.query_builder.conflict(Conflict::Replace);
        self
    }

    #[cfg(any(feature = "sqlite", feature = "postgres"))]
    pub fn conflict_columns(mut self, columns: &'a [&'a str]) -> Self {
        self.query_builder.add_conflict_columns(columns);
        self
    }

    pub async fn execute<E>(mut self, e: E) -> Result<DB::QueryResult, Error>
        where E: Executor<'a, Database = DB> {
        self.sql.push_str(self.query_builder.as_sql().as_str());
        Ok(sqlx::query_with(self.sql, self.arguments).execute(e).await?)
    }

}


#[cfg(any(feature = "postgres", feature = "sqlite"))]
impl<'a, T, DB, A> SetProvider<'a, DB> for Insert<'a, T, DB, A>
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

#[cfg(any(feature = "postgres", feature = "sqlite"))]
impl<'a, T, DB, A> Set<'a, DB> for Insert<'a, T, DB, A>
    where T: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}


#[cfg(any(feature = "postgres", feature = "sqlite"))]
impl<'a, T, DB, A> WhereProvider<'a, DB> for Insert<'a, T, DB, A>
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

#[cfg(any(feature = "postgres", feature = "sqlite"))]
impl<'a, T, DB, A> Where<'a, DB> for Insert<'a, T, DB, A>
    where T: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}
