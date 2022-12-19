use std::any::{Any, type_name, TypeId};
use sqlx::{Database, MySql, Postgres, Sqlite};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let target = TargetQuery::new::<Sqlite>();
        println!("{:?}", target);
    }
}