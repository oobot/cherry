use std::marker::PhantomData;

use anyhow::Error;
use futures_core::future::BoxFuture;
use sqlx::{Arguments, Database, Encode, Executor, Sqlite, Type};
use sqlx::database::HasArguments;

use crate::Cherry;
use crate::query_builder::{QueryBuilder, TargetQuery};
use crate::query_builder::delete::DeleteBuilder;
use crate::query_builder::end::section::EndSection;
use crate::query_builder::insert::InsertBuilder;
use crate::query_builder::select::SelectBuilder;
use crate::query_builder::set_clause::SetSection;
use crate::query_builder::update::UpdateBuilder;
use crate::query_builder::where_clause::condition::Condition;
use crate::query_internal::end::End;
use crate::query_internal::provider::{EndProvider, UpdateSetProvider, WhereProvider};
use crate::query_internal::r#where::Where;
use crate::query_internal::set::UpdateSet;
use crate::query_internal::set_column::SetColumn;
use crate::query_internal::where_column::WhereColumn;

// use futures_util::future::FutureExt;

pub struct Query<'a, T, DB: Database> {
    arguments: <DB as HasArguments<'a>>::Arguments,
    query_builder: QueryBuilder<'a>,
    _a: PhantomData<T>,
}

impl<'a, T, DB> Query<'a, T, DB>
    where
        T: Cherry<'a, DB>,
        DB: Database {

    pub fn new_insert(v: &'a T) -> Self {
        let mut arguments = <DB as HasArguments<'a>>::Arguments::default();
        v.arguments(&mut arguments);
        Self {
            arguments,
            query_builder: QueryBuilder::Insert(Self::create_insert_builder(1)),
            _a: Default::default(),
        }
    }

    pub fn new_insert_bulk(v: &'a [T]) -> Self {
        let mut arguments = <DB as HasArguments<'a>>::Arguments::default();
        v.iter().for_each(|row| row.arguments(&mut arguments));
        Self {
            arguments,
            query_builder: QueryBuilder::Insert(Self::create_insert_builder(v.len())),
            _a: Default::default(),
        }
    }

    fn create_insert_builder(rows_count: usize) -> InsertBuilder<'a> {
        InsertBuilder::from(
            TargetQuery::Sqlite,
            T::table(),
            T::columns().into_iter().map(|(_f, c)| c).collect(),
            rows_count,
        )
    }

    pub fn new_select() -> Self {
        Self {
            arguments: <DB as HasArguments<'a>>::Arguments::default(),
            query_builder: QueryBuilder::Select(
                SelectBuilder::from(TargetQuery::new::<DB>(), T::table())
            ),
            _a: Default::default(),
        }
    }

    pub fn new_update() -> Self {
        Self {
            arguments: <DB as HasArguments<'a>>::Arguments::default(),
            query_builder: QueryBuilder::Update(
                UpdateBuilder::from(TargetQuery::new::<DB>(), T::table())
            ),
            _a: Default::default(),
        }
    }

    pub fn new_delete() -> Self {
        Self {
            arguments: <DB as HasArguments<'a>>::Arguments::default(),
            query_builder: QueryBuilder::Delete(
                DeleteBuilder::from(TargetQuery::new::<DB>(), T::table())
            ),
            _a: Default::default(),
        }
    }

}


pub trait QueryExecutor<'a, T, DB> where T: Cherry<'a, DB>, DB: Database {

    fn execute<'e, E>(self, e: E) -> BoxFuture<'e, Result<DB::QueryResult, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database = DB> + 'e;


    fn one<'e, E>(self, e: E) -> BoxFuture<'e, Result<Option<T>, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database = DB> + 'e;

}

impl<'a, T> QueryExecutor<'a, T, Sqlite> for Query<'a, T, Sqlite>
    where
        T: Cherry<'a, Sqlite> {

    fn execute<'e, E>(self, e: E)
        -> BoxFuture<'e, Result<<Sqlite as Database>::QueryResult, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database=Sqlite> + 'e {
        Box::pin(async move {
            let sql = self.query_builder.as_sql();
            Ok(sqlx::query_with(sql.as_str(), self.arguments).execute(e).await?)
        })
    }

    fn one<'e, E>(self, e: E) -> BoxFuture<'e, Result<Option<T>, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database=Sqlite> + 'e {
        Box::pin(async move {
            let sql = self.query_builder.as_sql();
            let row = sqlx::query_with(sql.as_str(), self.arguments)
                .fetch_optional(e).await?;
            let t = match row {
                Some(row) => Some(T::from_row(&row)?),
                _ => None,
            };
            Ok(t)
        })
    }



}


impl<'a, T, DB> UpdateSetProvider<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn add_set_section(&mut self, section: SetSection<'a>) {
        self.query_builder.add_update_set(section);
    }
}

impl<'a, T, DB> UpdateSet<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}

impl<'a, T, DB> SetColumn<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}


impl<'a, T, DB> WhereProvider<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn add_where(&mut self, c: Condition<'a>) {
        self.query_builder.add_where(c);
    }

    fn surround_where(&mut self) {
        self.query_builder.surround_where();
    }

    fn take_surround(&mut self) -> Vec<Condition<'a>> {
        self.query_builder.take_surround()
    }
}

impl<'a, T, DB> Where<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}

impl<'a, T, DB> WhereColumn<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}


impl<'a, T, DB> EndProvider<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn add_end_section(&mut self, section: EndSection<'a>) {
        self.query_builder.add_end_section(section);
    }
}

impl<'a, T, DB> End<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}
