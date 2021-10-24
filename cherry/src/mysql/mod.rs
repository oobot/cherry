use anyhow::Error;
use sql_builder::SqlBuilder;
use sqlx::{MySql, MySqlPool, Transaction};
use sqlx::mysql::MySqlQueryResult;

use crate::{Arguments, Cherry, MySqlArguments, pools, WrapArguments, WrapRows};

pub mod template;

fn bulk_sql<T>(size: usize) -> SqlBuilder where T: Cherry {
    let columns = T::columns();
    let holders = vec!["?"; columns.len()];
    let mut sql = SqlBuilder::insert_into(T::table());
    sql.fields(columns.as_slice());
    (0..size).for_each(|_| {
        sql.values(holders.as_slice());
    });
    sql
}

fn set_arguments<T>(values: &[T]) -> Result<MySqlArguments, Error> where T: Cherry {
    let mut arg = WrapArguments::MySqlArguments(MySqlArguments::new());
    values.iter().for_each(|t| { t.arguments(&mut arg); } );
    arg.unwrap_mysql()
}

async fn execute<'a, S>(key: &str, sql: S, arguments: MySqlArguments<'a>,
                        tx: Option<&mut Transaction<'a, MySql>>)
                        -> Result<MySqlQueryResult, Error> where S: AsRef<str> {
    let x = match tx {
        Some(tx) => {
            sqlx::query_with(sql.as_ref(), arguments.inner).execute(tx).await?
        }
        _ => {
            let mut tx = pool(key)?.begin().await?;
            let x = sqlx::query_with(sql.as_ref(), arguments.inner)
                .execute(&mut tx)
                .await?;
            tx.commit().await?;
            x
        }
    };
    Ok(x)
}

async fn fetch<S: AsRef<str>, T>(key: &str, sql: S, arguments: MySqlArguments<'_>)
                                 -> Result<Vec<T>, Error> where T: Cherry + Sync {
    let output = sqlx::query_with(sql.as_ref(), arguments.inner)
        .fetch_all(pool(key)?)
        .await?;
    let mut vec = Vec::with_capacity(output.len());
    for row in output {
        vec.push(T::from_row(&WrapRows::MySqlRow(row))?);
    }
    Ok(vec)
}

fn pool(key: &str) -> Result<&MySqlPool, Error> {
    pools::get().mysql_pool.get(key).ok_or(anyhow!("No pool for key: {}", key))
}
