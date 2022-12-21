use std::any::{type_name, TypeId};
use std::borrow::BorrowMut;

use sqlx::{Database, MySql, Postgres, Sqlite};

use crate::sql::delete::DeleteBuilder;
use crate::sql::end::section::EndSection;
use crate::sql::insert::{Conflict, InsertBuilder};
use crate::sql::select::SelectBuilder;
use crate::sql::set_clause::SetSection;
use crate::sql::update::UpdateBuilder;
use crate::sql::where_clause::condition::Condition;

pub mod insert;
pub mod update;
pub mod select;
pub mod delete;
pub mod where_clause;
pub mod set_clause;
pub mod end;

#[derive(Debug, Copy, Clone)]
pub enum TargetQuery {
    MySql,
    Postgres,
    Sqlite,
}

impl TargetQuery {

    pub(crate) fn new<DB: Database>() -> Self {
        if TypeId::of::<DB>() == TypeId::of::<Sqlite>() {
            Self::Sqlite
        } else if TypeId::of::<DB>() == TypeId::of::<Postgres>() {
            Self::Postgres
        } else if TypeId::of::<DB>() == TypeId::of::<MySql>() {
            Self::MySql
        } else {
            panic!(r#"Unknown database type {}"#, type_name::<DB>());
        }
    }

    pub(crate) fn quote(&self, s: &str) -> String {
        match self {
            TargetQuery::MySql => format!("`{}`", s),
            TargetQuery::Postgres | TargetQuery::Sqlite => format!(r#""{}""#, s),
        }
    }
}


pub enum QueryBuilder<'a> {
    Insert(InsertBuilder<'a>),
    Update(UpdateBuilder<'a>),
    Delete(DeleteBuilder<'a>),
    Select(SelectBuilder<'a>),
}

impl<'a> QueryBuilder<'a> {
    pub(crate) fn as_sql(&self) -> String {
        match &self {
            QueryBuilder::Insert(b) => b.as_sql(),
            QueryBuilder::Update(b) => b.as_sql(),
            QueryBuilder::Delete(b) => b.as_sql(),
            QueryBuilder::Select(b) => b.as_sql(),
        }
    }

    pub(crate) fn add_where(&mut self, condition: Condition<'a>) {
        match self.borrow_mut() {
            QueryBuilder::Insert(b) => b.where_clause.add(condition),
            QueryBuilder::Update(b) => b.where_clause.add(condition),
            QueryBuilder::Delete(b) => b.where_clause.add(condition),
            QueryBuilder::Select(b) => b.where_clause.add(condition),
        }
    }

    pub(crate) fn surround_where(&mut self) {
        match self.borrow_mut() {
            QueryBuilder::Insert(b) => b.where_clause.make_temp(),
            QueryBuilder::Update(b) => b.where_clause.make_temp(),
            QueryBuilder::Delete(b) => b.where_clause.make_temp(),
            QueryBuilder::Select(b) => b.where_clause.make_temp(),
        }
    }

    pub(crate) fn take_surround(&mut self) -> Vec<Condition<'a>> {
        match self.borrow_mut() {
            QueryBuilder::Insert(b) => b.where_clause.take_temp(),
            QueryBuilder::Update(b) => b.where_clause.take_temp(),
            QueryBuilder::Delete(b) => b.where_clause.take_temp(),
            QueryBuilder::Select(b) => b.where_clause.take_temp(),
        }
    }

    pub(crate) fn add_update_set(&mut self, section: SetSection<'a>) {
        match self.borrow_mut() {
            QueryBuilder::Insert(b) => b.set_clause.add(section),
            QueryBuilder::Update(b) => b.set_clause.add(section),
            QueryBuilder::Delete(_) => (),
            QueryBuilder::Select(_) => (),
        }
    }

    pub(crate) fn add_end_section(&mut self, section: EndSection<'a>) {
        match self.borrow_mut() {
            QueryBuilder::Insert(_) => (),
            QueryBuilder::Update(_) =>(),
            QueryBuilder::Delete(_) => (),
            QueryBuilder::Select(b) => b.end.add(section),
        }
    }

    pub(crate) fn conflict(&mut self, conflict: Conflict) {
        match self.borrow_mut() {
            QueryBuilder::Insert(b) => b.conflict(conflict),
            _ => ()
        }
    }

    pub(crate) fn conflict_columns<I>(&mut self, columns: I)
        where I: IntoIterator<Item = &'a str> {
        match self.borrow_mut() {
            QueryBuilder::Insert(b) => b.set_conflict_columns(columns),
            _ => ()
        }
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let target = TargetQuery::new::<Sqlite>();
        println!("{:?}", target);
    }
}