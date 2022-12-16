use sqlx::{Database, Encode, Type};
use sqlx::database::HasArguments;

#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "sqlite")]
pub mod sqlite;
// #[cfg(feature = "mssql")]
// mod mssql;

// pub trait Arguments<'a, DB: HasArguments<'a>>: Sized + Send {
pub trait Arguments<'a, DB: Database>: Sized + Send {

    fn new() -> Self;

    fn add<T>(&mut self, v: T) where T: Encode<'a, DB> + Type<DB> + Send + 'a;

}
