use std::marker::PhantomData;
use std::mem;

use anyhow::Error;
use sqlx::{Database, Encode, Executor, IntoArguments, Type};

use crate::arguments::Arguments;
use crate::Cherry;
use crate::database::AboutDatabase;
use crate::query::end::End;
use crate::query::provider::{EndProvider, WhereProvider};
use crate::query::r#where::Where;
use crate::query_builder::end::EndClause;
use crate::query_builder::end::section::EndSection;
use crate::query_builder::where_clause::condition::Condition;
use crate::query_builder::where_clause::WhereClause;
use crate::query_builder::select::SelectBuilder;

pub struct Select<'a, C, DB, A> {
    arguments: A,
    sql: &'a mut String,
    query_builder: SelectBuilder<'a>,
    _a: PhantomData<C>,
    _b: PhantomData<&'a DB>,
}

impl<'a, C, DB, A> Select<'a, C, DB, A>
    where C: Cherry<'a, DB, A>,
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
            query_builder: SelectBuilder::from(DB::target(), C::table()),
            _a: Default::default(),
            _b: Default::default(),
        }
    }

    pub async fn one<'e, 'c: 'e, E>(mut self, e: E) -> Result<Option<C>, Error>
        where 'a: 'e,
              A: 'e,
              E: Executor<'c, Database = DB> {
        self.sql.push_str(&self.query_builder.sql());
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
    where C: Cherry<'a, DB, A>,
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

impl<'a, C, DB, A> EndProvider<'a, DB> for Select<'a, C, DB, A>
    where C: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

    fn add_value<V>(&mut self, v: V) where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.arguments.add(v);
    }

    fn add_end_section(&mut self, section: EndSection<'a>) {
        self.query_builder.end.add(section);
    }
}

impl<'a, C, DB, A> Where<'a, DB> for Select<'a, C, DB, A>
    where C: Cherry<'a, DB, A>,
          DB: Database,
          A: Arguments<'a, DB> + Send + 'a {

}

impl<'a, C, DB, A> End<'a, DB> for Select<'a, C, DB, A>
    where C: Cherry<'a, DB, A>,
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