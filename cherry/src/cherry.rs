use crate::arguments::WrapArguments;
use crate::error::CherryError;
use crate::rows::WrapRows;

pub trait Cherry: Sized + Send + Unpin {
    fn table() -> &'static str;
    fn columns() -> Vec<&'static str>;
    fn to_arguments(&self) -> WrapArguments;
    fn arguments<'a>(&'a self, arguments: &mut WrapArguments<'a>);
    fn from_row(rows: &WrapRows) -> Result<Self, CherryError>;
}
