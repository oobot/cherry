use std::any::{type_name, TypeId};

use sqlx::Database;

pub mod builder;
pub mod update_set_clause;
pub mod select_column;
pub mod where_clause;
pub mod where_condition;
pub mod end_clause;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum TargetDatabase {
    Sqlite,
    MySql,
    Postgres,
}

impl TargetDatabase {

    pub(crate) fn new<DB: Database>() -> Self {
        #[cfg(feature = "sqlite")]
        if TypeId::of::<DB>() == TypeId::of::<sqlx::Sqlite>() {
            return Self::Sqlite;
        }
        #[cfg(feature = "postgres")]
        if TypeId::of::<DB>() == TypeId::of::<sqlx::Postgres>() {
            return Self::Postgres;
        }
        #[cfg(feature = "mysql")]
        if TypeId::of::<DB>() == TypeId::of::<sqlx::MySql>() {
            return Self::MySql;
        }

        panic!(r#"Unknown database type {}"#, type_name::<DB>());
    }

    pub(crate) fn quote(&self, s: &str) -> String {
        match self {
            TargetDatabase::MySql => format!("`{}`", s),
            TargetDatabase::Postgres | TargetDatabase::Sqlite => format!(r#""{}""#, s),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SqlType {
    Insert,
    Update,
    Delete,
    Select,
}

#[derive(Copy, Clone)]
pub enum InsertOnConflict {
    None,
    Ignore,
    Update,
    Replace
}
