use std::marker::PhantomData;
use sqlx::{Database, Encode, IntoArguments, Type};

#[cfg(feature = "postgres")]
pub(crate) mod postgres;
#[cfg(feature = "mysql")]
pub(crate) mod mysql;
#[cfg(feature = "sqlite")]
pub(crate) mod sqlite;

// pub trait Arguments<'a, DB: HasArguments<'a>>: Sized + Send {
pub trait Arguments<'a, DB: Database>: IntoArguments<'a, DB> + Sized + Send {

    fn new() -> Self;

    fn add<T>(&mut self, v: T) where T: Encode<'a, DB> + Type<DB> + Send + 'a;

}

// pub enum WrapArguments<'a> {
//     #[cfg(feature = "sqlite")]
//     Sqlite(sqlx::sqlite::SqliteArguments<'a>, PhantomData<&'a ()>),
//
//     #[cfg(feature = "postgres")]
//     Postgres(sqlx::postgres::PgArguments, PhantomData<&'a ()>),
//
//     #[cfg(feature = "mysql")]
//     MySql(sqlx::mysql::MySqlArguments, PhantomData<&'a ()>),
// }
