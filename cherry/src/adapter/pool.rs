use std::marker::PhantomData;
use anyhow::{anyhow, Error};
use sqlx::{Arguments, IntoArguments};
use sqlx::database::HasArguments;
use sqlx::mysql::MySqlArguments;
use sqlx::sqlite::SqliteArguments;

use crate::adapter::Database;

pub struct Pool<DB: sqlx::Database> {
    // pub(crate) database: Database,
    // pub(crate) inner: InnerPool,
    pub(crate) inner: sqlx::Pool<DB>,
    // _marker: PhantomData<DB>,
}

/*
pub(crate) enum InnerPool {
    #[cfg(feature = "postgres")]
    PgPool(sqlx::Pool<sqlx::Postgres>),
    #[cfg(feature = "mysql")]
    MySqlPool(sqlx::Pool<sqlx::MySql>),
    #[cfg(feature = "sqlite")]
    SqlitePool(sqlx::Pool<sqlx::Sqlite>),
}*/

impl<DB: sqlx::Database> Pool<DB> {
    /*pub async fn connect(url: &str) -> Result<Self, Error> {
        let msg = format!("Unknown connect url: {}", url);
        match url.split_once(':').ok_or(anyhow!("{}", msg))?.0 {
            #[cfg(feature = "postgres")]
            "postgresql" => Ok(Self {
                database: Database::Postgres(sqlx::Postgres),
                inner: InnerPool::PgPool(sqlx::PgPool::connect(url).await?)
            }),
            #[cfg(feature = "mysql")]
            "mysql" => Ok(Self {
                database: Database::MySql(sqlx::MySql),
                inner: InnerPool::MySqlPool(sqlx::MySqlPool::connect(url).await?)
            }),
            #[cfg(feature = "sqlite")]
            "sqlite" => Ok(Self {
                database: Database::Sqlite(sqlx::Sqlite),
                inner: InnerPool::SqlitePool(sqlx::SqlitePool::connect(url).await?)
            }),
            _ => Err(anyhow!("{}", msg))?
        }
    }*/

    pub async fn connect(url: &str) -> Result<Self, Error> {
        let v = Self {
            inner: sqlx::Pool::<DB>::connect(url).await?
        };
        Ok(v)
    }

    pub fn arguments<'a>(&self) -> impl Arguments<'a, Database = DB> {
        <DB as HasArguments<'a>>::Arguments::default()
    }

    // pub fn arguments3<'a>(&self) -> SqliteArguments<'a> {
    //     <DB as HasArguments<'a>>::Arguments::default()
    // }



}

/*pub struct MyArguments<'a, DB> {
    sqlite: SqliteArguments<'a>,
    mysql: MySqlArguments,
    // pgsql: PgArguments,
    _marker: PhantomData<DB>,
}

impl<'a, DB: sqlx::Database> MyArguments<'a, DB> {
    pub fn new() -> Self {
        // create different arguments type by DB name
        todo!()
    }

    pub fn add<T: sqlx::Encode<'a, DB> + Send>(&mut self, v: T) {
        self.sqlite.add(v);
    }
}
*/


// pub struct ArgumentWrap<'a, DB: sqlx::Database> {
//     inner: Box<dyn Arguments<'a, Database = DB> + Sized>
// }

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqliteArguments;
    use super::*;

    #[async_std::test]
    async fn test() {
        let pool = Pool::<sqlx::Sqlite>::connect("sqlite::memory:").await.unwrap();
        let mut arguments = pool.arguments();
        arguments.add(1);
        arguments.add("text");

        // let arguments = arguments as SqliteArguments;
        // let arguments = SqliteArguments::from(arguments);

        // let a = sqlx::query_with("select 1", arguments)
        //     .execute(&pool.inner).await.unwrap();
    }

    fn accept<'a, DB: sqlx::Database>(a: impl Arguments<'a, Database = DB> + Sized) {

    }
}