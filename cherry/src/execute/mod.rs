use std::any::TypeId;

use crate::{connection, Result};
use crate::adapt::arguments::Arguments;
use crate::adapt::query_result::QueryResult;
use crate::adapt::transaction::Transaction;

pub(crate) mod wheres;

pub enum TxMode<'a> {
    Manual(&'a mut Transaction<'a>), Auto, None
}

pub struct Data<'a> {
    pub datasource: TypeId,
    pub sql: String,
    pub arguments: Arguments<'a>,
    pub tx: TxMode<'a>
}

pub async fn execute<'a>(data: Data<'a>) -> Result<QueryResult>  {
    let sql = data.sql.as_str();
    let arguments = data.arguments.inner;

    let result = match data.tx {
        TxMode::Manual(tx) => {
            sqlx::query_with(sql, arguments).execute(&mut tx.inner).await?
        },
        TxMode::Auto => {
            let mut tx = connection::get(data.datasource)?.inner.begin().await?;
            let result = sqlx::query_with(sql, arguments).execute(&mut tx).await?;
            tx.commit().await?;
            result
        },
        TxMode::None => {
            let pool = &connection::get(data.datasource)?.inner;
            sqlx::query_with(sql, arguments).execute(pool).await?
        },
    };

    Ok(QueryResult::from(result))
}
