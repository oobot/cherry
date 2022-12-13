use std::marker::PhantomData;
use std::mem;

use anyhow::Error;
use sqlx::{Database, Encode, Executor, IntoArguments, Type};

use crate::arguments::Arguments;
use crate::Cherry;
use crate::crud::end::End;
use crate::crud::provider::{EndProvider, WhereProvider};
use crate::crud::r#where::Where;
use crate::database::AboutDatabase;
use crate::statement::end::EndStatement;
use crate::statement::end::section::EndSection;
use crate::statement::r#where::condition::Condition;
use crate::statement::r#where::WhereStatement;
use crate::statement::select::SelectStatement;

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

    pub async fn one<'e, 'c: 'e, E>(mut self, e: E) -> Result<Option<C>, Error>
        where 'a: 'e,
              A: 'e,
              E: Executor<'c, Database = DB> {
        self.sql.push_str(&self.statement.sql());
        let row = sqlx::query_with(self.sql.as_str(), self.arguments)
            .fetch_optional(e).await?;
        let c = match row {
            Some(row) => Some(C::from_row(&row)?),
            _ => None,
        };
        Ok(c)
    }

}

impl<'a, C, DB, A> WhereProvider<'a, DB> for Select<'a, C, DB, A>
    where C: Cherry<DB>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn make_wrap(&mut self) {
        self.statement.r#where.make_temp();
    }

    fn take_wrap(&mut self) -> Vec<Condition<'a>> {
        self.statement.r#where.take_temp()
    }

    fn add_statement(&mut self, c: Condition<'a>) {
        self.statement.r#where.add(c);
    }


    // fn where_statement(&mut self) -> &mut WhereStatement<'a> {
    //     &mut self.statement.r#where
    // }
}

impl<'a, C, DB, A> EndProvider<'a, DB> for Select<'a, C, DB, A>
    where C: Cherry<DB>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn add_section(&mut self, section: EndSection<'a>) {
        self.statement.end.add(section);
    }
}

impl<'a, C, DB, A> Where<'a, DB> for Select<'a, C, DB, A>
    where C: Cherry<DB>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}

impl<'a, C, DB, A> End<'a, DB> for Select<'a, C, DB, A>
    where C: Cherry<DB>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}