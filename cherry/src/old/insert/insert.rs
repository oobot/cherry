use std::marker::PhantomData;

use anyhow::Error;
use sqlx::{Database, Encode, Executor, Type};

use crate::arguments::Arguments;
use crate::Cherry;
use crate::query_builder::insert::{Conflict, InsertBuilder};
use crate::query_builder::set_clause::SetSection;
use crate::query_builder::TargetQuery;
use crate::query_internal::insert::insert_set::InsertSet;
use crate::query_internal::provider::SetProvider;
use crate::query_internal::set::UpdateSet;

pub struct Insert<'a, T, DB, A> {
    arguments: A,
    sql: &'a mut String,
    query_builder: InsertBuilder<'a>,
    _a: PhantomData<DB>,
    _b: PhantomData<T>,
}

impl<'a, T, DB, A> Insert<'a, T, DB, A>
    where T: Cherry<'a, DB, A> + 'a,
          DB: Database,
          A: Arguments<'a, DB> + Send +'a {

    pub fn from_one(v: &'a T, sql: &'a mut String) -> Self {
        assert!(sql.is_empty());
        let mut arguments = A::new();
        v.arguments(&mut arguments);
        Self {
            arguments, sql,
            query_builder: Self::create_query_builder(1),
            _a: Default::default(), _b: Default::default(),
        }
    }

    pub fn from_multiple(v: &'a [T], sql: &'a mut String) -> Self {
        assert!(sql.is_empty());
        let mut arguments = A::new();
        v.iter().for_each(|row| row.arguments(&mut arguments));
        Self {
            arguments, sql,
            query_builder: Self::create_query_builder(v.len()),
            _a: Default::default(), _b: Default::default(),
        }
    }

    fn create_query_builder(rows_count: usize) -> InsertBuilder<'a> {
        InsertBuilder::from(
            TargetQuery::new::<DB>(),
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

    pub async fn execute<E>(self, e: E) -> Result<DB::QueryResult, Error>
        where E: Executor<'a, Database = DB> {
        self.sql.push_str(self.query_builder.as_sql().as_str());
        Ok(sqlx::query_with(self.sql, self.arguments).execute(e).await?)
    }

}


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

impl<'a, T, DB, A> UpdateSet<'a, DB> for Insert<'a, T, DB, A>
    where T: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}

impl<'a, T, DB, A> InsertSet<'a, DB> for Insert<'a, T, DB, A>
    where T: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}


#[cfg(any(feature = "postgres", feature = "sqlite"))]
pub mod where_filter {
    use sqlx::{Database, Encode, Type};

    use crate::arguments::Arguments;
    use crate::Cherry;
    use crate::query_builder::where_clause::condition::Condition;
    use crate::query_internal::insert::insert::Insert;
    use crate::query_internal::insert::insert_where::InsertWhere;
    use crate::query_internal::provider::WhereProvider;
    use crate::query_internal::r#where::Where;

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

    impl<'a, T, DB, A> Where<'a, DB> for Insert<'a, T, DB, A>
        where T: Cherry<'a, DB, A>,
              DB: Database,
              A: Arguments<'a, DB> + Send + 'a {

    }

    impl<'a, T, DB, A> InsertWhere<'a, DB> for Insert<'a, T, DB, A>
        where T: Cherry<'a, DB, A>,
              DB: Database,
              A: Arguments<'a, DB> + Send + 'a {

    }
}
