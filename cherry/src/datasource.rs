use std::any::type_name;

use async_trait::async_trait;

use crate::{Cherry, connection};
use crate::connection::PoolConfig;
use crate::query::delete::Delete;
use crate::query::insert::Insert;
use crate::query::insert_update::InsertUpdate;
use crate::query::raw_sql::RawSql;
use crate::query::select::Select;
use crate::query::update::Update;
use crate::types::{Result, Transaction};

#[async_trait]
pub trait DataSource {

    async fn setup(conn: PoolConfig) -> Result<()> {
        connection::add_conn(type_name::<Self>(), conn).await
    }

    fn insert<'a, T>(v: &'a T) -> Insert<'a> where T: Cherry {
        Insert::insert(type_name::<Self>(),  v)
    }

    fn insert_bulk<'a, T>(v: &'a [T]) -> Insert<'a> where T: Cherry {
        Insert::insert_bulk(type_name::<Self>(), v)
    }

    fn insert_ignore<'a, T>(v: &'a [T]) -> Insert<'a> where T: Cherry {
        Insert::insert_ignore(type_name::<Self>(), v)
    }

    fn insert_replace<'a, T>(v: &'a [T]) -> Insert<'a> where T: Cherry {
        Insert::insert_replace(type_name::<Self>(), v)
    }

    fn insert_update<'a, T>(v: &'a [T]) -> InsertUpdate<'a> where T: Cherry {
        InsertUpdate::insert_update(type_name::<Self>(), v)
    }

    fn delete<'a, T>() -> Delete<'a> where T: Cherry {
        Delete::new::<T>(type_name::<Self>())
    }

    fn update<'a, T>() -> Update<'a> where T: Cherry {
        Update::new::<T>(type_name::<Self>())
    }

    fn select<'a, T>() -> Select<'a, T> where T: Cherry {
        Select::new(type_name::<Self>())
    }

    fn raw_sql<'a>() -> RawSql<'a> {
        RawSql::new(type_name::<Self>())
    }

    async fn begin<'a>() -> Result<Transaction<'a>> {
        Ok(connection::get(type_name::<Self>())?.begin().await?)
    }

}


#[cfg(test)]
mod test {

    #[async_test]
    async fn test() {

    }
}