use std::marker::PhantomData;

use sqlx::{Database, Encode, Executor, IntoArguments, Type};

use crate::{Cherry, Error};
use crate::arguments::Arguments;
use crate::database::AboutDatabase;
use crate::query::provider::WhereProvider;
use crate::query::r#where::Where;
use crate::query_builder::insert::{Conflict, InsertBuilder};
use crate::query_builder::r#where::condition::Condition;

enum Data<'a, T> {
    One(&'a T),
    Multiple(&'a [T])
}

pub struct Insert<'a, T, DB, A> {
    data: Data<'a, T>,
    arguments: A,
    sql: &'a mut String,
    query_builder: InsertBuilder<'a>,
    _a: PhantomData<DB>,
    // _b: PhantomData<A>,
    _c: PhantomData<T>,
}

impl<'a, T, DB, A> Insert<'a, T, DB, A>
    where T: Cherry<DB> + 'a,
          DB: Database + AboutDatabase<'a, DB, A>,
          A: Arguments<'a, DB> + IntoArguments<'a, DB> + Send +'a {

    fn from(data: Data<'a, T>, sql: &'a mut String) -> Self {
        assert!(sql.is_empty());
        let rows = match &data { Data::One(_) => 1, Data::Multiple(v) => v.len(), };
        Self {
            data, arguments: DB::arguments(), sql,
            query_builder: InsertBuilder::from(
                DB::database(),
                T::table(),
                T::columns().into_iter().map(|(_f, c)| c).collect(),
                rows,
            ),
            _a: Default::default(), _c: Default::default()
        }
    }

    pub fn from_one(v: &'a T, sql: &'a mut String) -> Self {
        Self::from(Data::One(v), sql)
    }

    pub fn from_multiple(v: &'a [T], sql: &'a mut String) -> Self {
        Self::from(Data::Multiple(v), sql)
    }

    pub fn on_conflict_ignore(mut self) -> Self {
        self.query_builder.conflict(Conflict::Ignore);
        self
    }

    #[cfg(any(feature = "mysql", feature = "postgres"))]
    pub fn on_conflict_update(mut self) -> Self {
        self.query_builder.conflict(Conflict::Update);
        self
    }

    #[cfg(any(feature = "mysql", feature = "sqlite"))]
    pub fn on_conflict_replace(mut self) -> Self {
        self.query_builder.conflict(Conflict::Replace);
        self
    }

    #[cfg(feature = "postgres")]
    pub fn conflict_columns(mut self, columns: &'a [&'a str]) -> Self {
        self.query_builder.add_conflict_columns(columns);
        self
    }

    #[cfg(any(feature = "mysql", feature = "postgres"))]
    pub fn update_columns(mut self, columns: &'a [&'a str]) -> Self {
        self.query_builder.add_conflict_columns(columns);
        self
    }

    // #[cfg(any(feature = "mysql", feature = "postgres"))]
    // pub fn update_value<V>(mut self, c: &'a str, v: V)
    //     where V: Encode<'a, DB> + Type<DB> + Send + 'a {
    //
    // }

    pub async fn execute<E>(mut self, e: E) -> Result<DB::QueryResult, Error>
        where E: Executor<'a, Database = DB> {
        let arguments = DB::arguments();
        let sql = "";

        Ok(sqlx::query_with(sql, arguments).execute(e).await?)
    }

}

// TODO Wait for Update traits to update other values
// #[cfg(any(feature = "mysql", feature = "postgres"))]

#[cfg(feature = "postgres")]
impl<'a, T, DB, A> WhereProvider<'a, DB> for Insert<'a, T, DB, A>
    where T: Cherry<DB>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn make_wrap(&mut self) {
        self.query_builder.r#where.make_temp();
    }

    fn take_wrap(&mut self) -> Vec<Condition<'a>> {
        self.query_builder.r#where.take_temp()
    }

    fn add_statement(&mut self, c: Condition<'a>) {
        self.query_builder.r#where.add(c);
    }
}

#[cfg(feature = "postgres")]
impl<'a, T, DB, A> Where<'a, DB> for Insert<'a, T, DB, A>
    where T: Cherry<DB>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}
