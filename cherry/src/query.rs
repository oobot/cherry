use std::marker::PhantomData;

use sqlx::{Arguments, Database, Encode, Type};
use sqlx::database::HasArguments;

use crate::Cherry;
use crate::clause::end::End;
use crate::clause::set_column::SetColumn;
use crate::clause::set_value::SetValue;
use crate::clause::where_column::WhereColumn;
use crate::clause::where_value::Where;
use crate::provider::{EndProvider, SetValueProvider, WhereProvider};
use crate::sql::{QueryBuilder, TargetQuery};
use crate::sql::delete::DeleteBuilder;
use crate::sql::end::section::EndSection;
use crate::sql::insert::{Conflict, InsertBuilder};
use crate::sql::select::SelectBuilder;
use crate::sql::set_clause::SetSection;
use crate::sql::update::UpdateBuilder;
use crate::sql::where_clause::condition::Condition;

pub struct Query<'a, T, DB: Database> {
    pub(crate) arguments: <DB as HasArguments<'a>>::Arguments,
    pub(crate) query_builder: QueryBuilder<'a>,
    _a: PhantomData<T>,
}

impl<'a, T, DB> Query<'a, T, DB>
    where
        T: Cherry<'a, DB>,
        DB: Database {

    pub(crate) fn new_insert(v: &'a T) -> Self {
        let mut arguments = <DB as HasArguments<'a>>::Arguments::default();
        v.arguments(&mut arguments);
        Self {
            arguments,
            query_builder: QueryBuilder::Insert(Self::create_insert_builder(1)),
            _a: Default::default(),
        }
    }

    pub(crate) fn new_insert_bulk(v: &'a [T]) -> Self {
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

    pub(crate) fn new_select() -> Self {
        Self {
            arguments: <DB as HasArguments<'a>>::Arguments::default(),
            query_builder: QueryBuilder::Select(
                SelectBuilder::from(TargetQuery::new::<DB>(), T::table())
            ),
            _a: Default::default(),
        }
    }

    pub(crate) fn new_update() -> Self {
        Self {
            arguments: <DB as HasArguments<'a>>::Arguments::default(),
            query_builder: QueryBuilder::Update(
                UpdateBuilder::from(TargetQuery::new::<DB>(), T::table())
            ),
            _a: Default::default(),
        }
    }

    pub(crate) fn new_delete() -> Self {
        Self {
            arguments: <DB as HasArguments<'a>>::Arguments::default(),
            query_builder: QueryBuilder::Delete(
                DeleteBuilder::from(TargetQuery::new::<DB>(), T::table())
            ),
            _a: Default::default(),
        }
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
    pub fn conflict_column(mut self, column: &'a str) -> Self {
        self.query_builder.conflict_columns([column]);
        self
    }

    #[cfg(any(feature = "sqlite", feature = "postgres"))]
    pub fn conflict_columns<I>(mut self, columns: I) -> Self
        where
            I: IntoIterator<Item = &'a str> {
        self.query_builder.conflict_columns(columns);
        self
    }

}


impl<'a, T, DB> SetValueProvider<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn add_set_section(&mut self, section: SetSection<'a>) {
        self.query_builder.add_update_set(section);
    }
}

impl<'a, T, DB> SetValue<'a, DB> for Query<'a, T, DB>
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
