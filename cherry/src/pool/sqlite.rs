use anyhow::Error;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use sqlx::{Database, Describe, Either, Execute, Executor, Pool, Sqlite};
use sqlx::database::HasStatement;

use crate::pool::PoolOptions;

#[derive(Debug)]
pub struct SqlitePool(pub(crate) Pool<Sqlite>);

impl SqlitePool {
    pub async fn connect(url: &str) -> Result<Self, Error> {
        Ok(Self ( Pool::<Sqlite>::connect(url).await? ))
    }

    pub async fn from(opts: PoolOptions) -> Result<Self, Error> {
        Ok(Self ( opts.to_pool().await? ))
    }

}

impl<'c> Executor<'c> for &'_ SqlitePool {
    type Database = Sqlite;

    fn execute<'e, 'q: 'e, E: 'q>(self, query: E)
        -> BoxFuture<'e, Result<<Self::Database as Database>::QueryResult, sqlx::Error>>
        where 'c: 'e, E: Execute<'q, Self::Database> {
        self.0.execute(query)
    }

    fn execute_many<'e, 'q: 'e, E: 'q>(self, query: E)
        -> BoxStream<'e, Result<<Self::Database as Database>::QueryResult, sqlx::Error>>
        where 'c: 'e, E: Execute<'q, Self::Database> {
        self.0.execute_many(query)
    }

    fn fetch<'e, 'q: 'e, E: 'q>(self, query: E)
        -> BoxStream<'e, Result<<Self::Database as Database>::Row, sqlx::Error>>
        where 'c: 'e, E: Execute<'q, Self::Database> {
        self.0.fetch(query)
    }

    fn fetch_many<'e, 'q: 'e, E: 'q>(self, query: E)
        -> BoxStream<'e, Result<Either<<Self::Database as Database>::QueryResult, <Self::Database as Database>::Row>, sqlx::Error>>
        where 'c: 'e, E: Execute<'q, Self::Database> {
        self.0.fetch_many(query)
    }

    fn fetch_all<'e, 'q: 'e, E: 'q>(self, query: E)
        -> BoxFuture<'e, Result<Vec<<Self::Database as Database>::Row>, sqlx::Error>>
        where 'c: 'e, E: Execute<'q, Self::Database> {
        self.0.fetch_all(query)
    }

    fn fetch_one<'e, 'q: 'e, E: 'q>(self, query: E)
        -> BoxFuture<'e, Result<<Self::Database as Database>::Row, sqlx::Error>>
        where 'c: 'e, E: Execute<'q, Self::Database> {
        self.0.fetch_one(query)
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(self, query: E)
        -> BoxFuture<'e, Result<Option<<Self::Database as Database>::Row>, sqlx::Error>>
        where 'c: 'e, E: Execute<'q, Self::Database> {
        self.0.fetch_optional(query)
    }

    fn prepare<'e, 'q: 'e>(self, query: &'q str)
        -> BoxFuture<'e, Result<<Self::Database as HasStatement<'q>>::Statement, sqlx::Error>>
        where 'c: 'e {
        self.0.prepare(query)
    }

    fn prepare_with<'e, 'q: 'e>(self, sql: &'q str, parameters: &'e [<Self::Database as Database>::TypeInfo])
        -> BoxFuture<'e, Result<<Self::Database as HasStatement<'q>>::Statement, sqlx::Error>>
        where 'c: 'e {
        self.0.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(self, sql: &'q str)
        -> BoxFuture<'e, Result<Describe<Self::Database>, sqlx::Error>>
        where 'c: 'e {
        self.0.describe(sql)
    }
}
