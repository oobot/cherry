use anyhow::Error;
use futures_core::future::BoxFuture;
use sqlx::{Database, Executor, FromRow};

use crate::Cherry;
use crate::query::Query;

pub trait QueryExecutor<'a, T, DB> where T: Cherry<'a, DB>, DB: Database {

    fn execute<'e, E>(self, e: E) -> BoxFuture<'e, Result<DB::QueryResult, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database = DB> + 'e;


    fn one<'e, E>(self, e: E) -> BoxFuture<'e, Result<Option<T>, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database = DB> + 'e;

    fn all<'e, E>(self, e: E) -> BoxFuture<'e, Result<Vec<T>, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database = DB> + 'e;

    // implement FromRow for tuples of types that implement Decode
    // up to tuples of 16 values
    fn tuple<'e, O, E>(self, e: E) -> BoxFuture<'e, Result<Option<O>, Error>>
        where
            'a: 'e,
            O: Send + Unpin + for<'r> FromRow<'r, DB::Row> + 'e,
            E: Executor<'e, Database = DB> + 'e;

    fn tuples<'e, O, E>(self, e: E) -> BoxFuture<'e, Result<Vec<O>, Error>>
        where
            'a: 'e,
            O: Send + Unpin + for<'r> FromRow<'r, DB::Row> + 'e,
            E: Executor<'e, Database = DB> + 'e;

}


macro_rules! gen_executor {
    ($db: ty) => {

impl<'a, T> QueryExecutor<'a, T, $db> for Query<'a, T, $db>
    where
        T: Cherry<'a, $db> {

    fn execute<'e, E>(self, e: E)
                      -> BoxFuture<'e, Result<<$db as Database>::QueryResult, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database=$db> + 'e {
        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            Ok(sqlx::query_with(&sql, self.arguments).execute(e).await?)
        })
    }

    fn one<'e, E>(self, e: E) -> BoxFuture<'e, Result<Option<T>, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database=$db> + 'e {
        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            let row = sqlx::query_with(&sql, self.arguments)
                .fetch_optional(e).await?;
            let t = match row {
                Some(row) => Some(T::from_row(&row)?),
                _ => None,
            };
            Ok(t)
        })
    }

    fn all<'e, E>(self, e: E) -> BoxFuture<'e, Result<Vec<T>, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database=$db> + 'e {

        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            let rows = sqlx::query_with(&sql, self.arguments)
                .fetch_all(e).await?;
            let mut vec = Vec::with_capacity(rows.len());
            for row in rows {
                vec.push(T::from_row(&row)?);
            }
            Ok(vec)
        })
    }

    fn tuple<'e, O, E>(self, e: E) -> BoxFuture<'e, Result<Option<O>, Error>>
        where
            'a: 'e,
            O: Send + Unpin + for<'r> FromRow<'r, <$db as Database>::Row> + 'e,
            E: Executor<'e, Database=$db> + 'e {

        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            let row = sqlx::query_with(&sql, self.arguments)
                .fetch_optional(e).await?;
            let tuple = match row {
                Some(row) => O::from_row(&row).map(Some),
                _ => Ok(None),
            };
            Ok(tuple?)
        })
    }

    fn tuples<'e, O, E>(self, e: E) -> BoxFuture<'e, Result<Vec<O>, Error>>
        where
            'a: 'e,
            O: Send + Unpin + for<'r> FromRow<'r, <$db as Database>::Row> + 'e,
            E: Executor<'e, Database=$db> + 'e {

        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            let rows = sqlx::query_with(&sql, self.arguments)
                .fetch_all(e).await?;

            let mut vec = Vec::with_capacity(rows.len());
            for row in rows {
                vec.push(O::from_row(&row)?);
            }

            Ok(vec)
        })
    }


}

    };
}

#[cfg(feature = "sqlite")]
gen_executor!(sqlx::Sqlite);
#[cfg(feature = "postgres")]
gen_executor!(sqlx::Postgres);
#[cfg(feature = "mysql")]
gen_executor!(sqlx::MySql);

/*
#[cfg(feature = "sqlite")]
impl<'a, T> QueryExecutor<'a, T, sqlx::Sqlite> for Query<'a, T, sqlx::Sqlite>
    where
        T: Cherry<'a, sqlx::Sqlite> {

    fn execute<'e, E>(self, e: E)
                      -> BoxFuture<'e, Result<<sqlx::Sqlite as Database>::QueryResult, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database=sqlx::Sqlite> + 'e {
        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            Ok(sqlx::query_with(&sql, self.arguments).execute(e).await?)
        })
    }

    fn one<'e, E>(self, e: E) -> BoxFuture<'e, Result<Option<T>, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database=sqlx::Sqlite> + 'e {
        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            let row = sqlx::query_with(&sql, self.arguments)
                .fetch_optional(e).await?;
            let t = match row {
                Some(row) => Some(T::from_row(&row)?),
                _ => None,
            };
            Ok(t)
        })
    }

    fn all<'e, E>(self, e: E) -> BoxFuture<'e, Result<Vec<T>, Error>>
        where
            'a: 'e,
            E: Executor<'e, Database=sqlx::Sqlite> + 'e {

        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            let rows = sqlx::query_with(&sql, self.arguments)
                .fetch_all(e).await?;
            let mut vec = Vec::with_capacity(rows.len());
            for row in rows {
                vec.push(T::from_row(&row)?);
            }
            Ok(vec)
        })
    }

    fn tuple<'e, O, E>(self, e: E) -> BoxFuture<'e, Result<Option<O>, Error>>
        where
            'a: 'e,
            O: Send + Unpin + for<'r> FromRow<'r, <sqlx::Sqlite as Database>::Row> + 'e,
            E: Executor<'e, Database=sqlx::Sqlite> + 'e {

        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            let row = sqlx::query_with(&sql, self.arguments)
                .fetch_optional(e).await?;
            let tuple = match row {
                Some(row) => O::from_row(&row).map(Some),
                _ => Ok(None),
            };
            Ok(tuple?)
        })
    }

    fn tuples<'e, O, E>(self, e: E) -> BoxFuture<'e, Result<Vec<O>, Error>>
        where
            'a: 'e,
            O: Send + Unpin + for<'r> FromRow<'r, <sqlx::Sqlite as Database>::Row> + 'e,
            E: Executor<'e, Database=sqlx::Sqlite> + 'e {

        Box::pin(async move {
            let sql = self.sql_builder.as_sql();
            let rows = sqlx::query_with(&sql, self.arguments)
                .fetch_all(e).await?;

            let mut vec = Vec::with_capacity(rows.len());
            for row in rows {
                vec.push(O::from_row(&row)?);
            }

            Ok(vec)
        })
    }

}
*/