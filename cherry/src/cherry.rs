use sqlx::Database;

use crate::arguments::Arguments;
use crate::Result;
use crate::row::Row;

pub trait Cherry: Sized + Send + Unpin {
    type Database: Database;
    fn table() -> &'static str;
    fn columns() -> Vec<&'static str>;
    fn arguments<'a>(&'a self, arguments: &mut Arguments<'a, Self::Database>);
    fn from_row(row: &Row<Self::Database>) -> Result<Self>;
}
