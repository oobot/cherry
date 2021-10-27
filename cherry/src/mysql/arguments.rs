use sqlx::database::HasArguments;
use sqlx::MySql;
use sqlx::encode::Encode;
use sqlx::types::Type;

use sqlx::Arguments as SqlxArguments;
use crate::arguments::Arguments;
use crate::impl_arguments;

pub struct MySqlArguments<'q> {
    pub(crate) inner: <MySql as HasArguments<'q>>::Arguments,
    pub(crate) count: usize,
}

impl_arguments!(MySql, MySqlArguments<'q>);
