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
