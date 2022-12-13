use std::marker::PhantomData;
use std::mem;

use anyhow::Error;
use sqlx::{Database, Encode, Executor, IntoArguments, Type};

use crate::arguments::Arguments;
use crate::Cherry;
use crate::database::AboutDatabase;
use crate::sql::condition::{Condition, Ending};
use crate::sql::filter::Filter;
use crate::sql::filter_statement::FilterStatement;
use crate::sql::select_statement::SelectStatement;

pub struct Select<'a, C, DB, A> {
    arguments: A,
    sql: &'a mut String,
    statement: SelectStatement<'a>,
    _a: PhantomData<C>,
    _b: PhantomData<&'a DB>,
}

impl<'a, C, DB, A> Select<'a, C, DB, A>
    where C: Cherry<DB>,
          DB: Database + AboutDatabase<'a, DB, A>,
          A: Arguments<'a, DB> + IntoArguments<'a, DB> + Send + 'a {

    /// FIXME: Should have a better solution. (a `str` container with lifetime parameter?)
    /// Because of the `Select::new` was called and created outside, the `'a` lifetime assign by the caller.
    /// `sqlx::query_with(sql, arguments)` need `sql` live as long as `arguments`,
    /// so the empty sql container created by the caller.
    pub fn new(sql: &'a mut String) -> Self {
        assert!(sql.is_empty());
        Self {
            arguments: DB::arguments(),
            sql,
            statement: SelectStatement::from(<C as Cherry<DB>>::table()),
            _a: Default::default(),
            _b: Default::default(),
        }
    }

    pub fn by_id<T>(mut self, v: T) -> Self where T: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
        self
    }

    pub async fn one<'e, 'c: 'e, E>(mut self, e: E) -> Result<Option<C>, Error>
        where 'a: 'e,
              A: 'e,
              E: Executor<'c, Database = DB> {
        let table = <C as Cherry<DB>>::table();
        let sql = format!("select * from {} where id = ?", table);
        self.sql.push_str(sql.as_str());

        let row = sqlx::query_with(self.sql.as_str(), self.arguments)
            .fetch_optional(e).await?;
        let c = match row {
            Some(row) => Some(C::from_row(&row)?),
            _ => None,
        };
        Ok(c)
    }

}

impl<'a, C, DB, A> Filter<'a, DB> for Select<'a, C, DB, A>
    where C: Cherry<DB>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn filter(&mut self) -> &mut FilterStatement<'a> {
        &mut self.statement.filter
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}