use crate::arguments::WrapArguments;
use crate::Result;
use crate::row::Row;

pub trait Cherry: Sized + Send + Unpin {
    fn table() -> &'static str;
    fn columns() -> Vec<&'static str>;
    fn arguments<'a>(&'a self, arguments: &mut WrapArguments<'a>);
    fn from_row(row: &Row) -> Result<Self>;
}
