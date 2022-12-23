use std::marker::PhantomData;

use sqlx::{Arguments, Database, Encode, Type};
use sqlx::database::HasArguments;

use crate::Cherry;
use crate::clause::{End, InsertConflict, UpdateSet, Where, WhereColumn};
use crate::clause::select_column::SelectColumn;
use crate::provider::Provider;
use crate::sql::builder::SqlBuilder;
use crate::sql::TargetDatabase;

pub struct Query<'a, T, DB: Database> {
    pub(crate) arguments: <DB as HasArguments<'a>>::Arguments,
    pub(crate) sql_builder: SqlBuilder<'a>,
    _a: PhantomData<T>,
}

impl<'a, T, DB> Query<'a, T, DB>
    where
        T: Cherry<'a, DB>,
        DB: Database {

    pub(crate) fn new_insert(v: &'a T) -> Self {
        let mut arguments = <DB as HasArguments<'a>>::Arguments::default();
        v.arguments(&mut arguments);
        Self::create_insert(arguments, 1)
    }

    pub(crate) fn new_insert_bulk(v: &'a [T]) -> Self {
        let mut arguments = <DB as HasArguments<'a>>::Arguments::default();
        v.iter().for_each(|row| row.arguments(&mut arguments));
        Self::create_insert(arguments, v.len())
    }

    fn create_insert(arguments: <DB as HasArguments<'a>>::Arguments, rows_count: usize) -> Self {
        Self {
            arguments,
            sql_builder: SqlBuilder::from_insert(
                TargetDatabase::new::<DB>(),
                T::table(),
                T::columns().into_iter().map(|(_f, c)| c).collect(),
                rows_count,
            ),
            _a: Default::default(),
        }
    }

    pub(crate) fn new_select() -> Self {
        Self {
            arguments: <DB as HasArguments<'a>>::Arguments::default(),
            sql_builder: SqlBuilder::from_select(
                TargetDatabase::new::<DB>(),
                T::table(),
                T::columns().into_iter().map(|(_f, c)| c).collect()
            ),
            _a: Default::default(),
        }
    }

    pub(crate) fn new_update() -> Self {
        Self {
            arguments: <DB as HasArguments<'a>>::Arguments::default(),
            sql_builder: SqlBuilder::from_update(TargetDatabase::new::<DB>(), T::table()),
            _a: Default::default(),
        }
    }

    pub(crate) fn new_delete() -> Self {
        Self {
            arguments: <DB as HasArguments<'a>>::Arguments::default(),
            sql_builder: SqlBuilder::from_delete(TargetDatabase::new::<DB>(), T::table()),
            _a: Default::default(),
        }
    }

}

impl<'a, T, DB> Provider<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn sql_builder(&mut self) -> &mut SqlBuilder<'a> {
        &mut self.sql_builder
    }
}

impl<'a, T, DB> InsertConflict<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}

impl<'a, T, DB> UpdateSet<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}

impl<'a, T, DB> SelectColumn<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}

impl<'a, T, DB> Where<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}

impl<'a, T, DB> WhereColumn<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}

impl<'a, T, DB> End<'a, DB> for Query<'a, T, DB>
    where T: Cherry<'a, DB> + 'a,
          DB: Database {

}
