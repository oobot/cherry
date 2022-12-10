use sqlx::{Database, Encode, Type};
use sqlx::database::HasArguments;

// #[cfg(feature = "postgres")]
// pub use postgres::*;
// #[cfg(feature = "mysql")]
// pub use mysql::*;
// #[cfg(feature = "sqlite")]
// pub use sqlite::*;
// #[cfg(feature = "mssql")]
// pub use mssql::*;

#[cfg(feature = "postgres")]
pub(crate) mod postgres;
#[cfg(feature = "mysql")]
pub(crate) mod mysql;
#[cfg(feature = "sqlite")]
pub(crate) mod sqlite;
// #[cfg(feature = "mssql")]
// mod mssql;

// pub trait QueryArguments<'a>: Sized + Send {
//     fn take(self) -> Self;
// }

// pub trait Arguments<'a, DB: HasArguments<'a>>: Sized + Send {
pub trait Arguments<'a, DB: Database>: Sized + Send {

    fn new() -> Self;

    fn raw(self) -> <DB as HasArguments<'a>>::Arguments;

    fn add<T>(&mut self, v: T) where T: Encode<'a, DB> + Type<DB> + Send + 'a;

}
