// #![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code, unreachable_code)]

pub use {
    anyhow::Error,
    cherry::Cherry,
    database::AboutDatabase,
    pool::PoolOptions,
    arguments::Arguments,
};

pub mod query {
    pub use crate::query_internal::{
        insert::insert::Insert,
        select::select::Select,
        update::update::Update,
        delete::delete::Delete,
        // provider::{SetProvider, WhereProvider, EndProvider},
        set::UpdateSet,
        r#where::Where,
        end::End,
    };

    // pub(crate) use crate::query_internal::provider::{SetProvider, WhereProvider, EndProvider};
}

#[cfg(feature = "sqlite")]
pub mod sqlite {
    pub use crate::pool::sqlite::SqlitePool;
    pub use crate::arguments::sqlite::SqliteArguments;
}

#[cfg(feature = "postgres")]
pub mod postgres {
    pub use crate::pool::postgres::PgPool;
    pub use crate::arguments::postgres::PgArguments;
}

#[cfg(feature = "mysql")]
pub mod mysql {
    pub use crate::pool::mysql::MySqlPool;
    pub use crate::arguments::mysql::MySqlArguments;
}

pub mod sqlx {
    pub use sqlx::*;
}

pub(crate) mod cherry;
pub(crate) mod database;
pub(crate) mod pool;
pub(crate) mod arguments;
pub(crate) mod query_internal;
pub(crate) mod query_builder;


#[cfg(not(any(feature = "mysql", feature = "postgres", feature = "sqlite")))]
compile_error!("one of the features ['mysql', 'postgres', 'sqlite'] must be enabled");
