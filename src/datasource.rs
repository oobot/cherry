use std::any::Any;

use async_trait::async_trait;

use crate::{Cherry, connection};
use crate::query::delete::Delete;
use crate::query::insert::Insert;
use crate::query::insert_update::InsertUpdate;
use crate::query::select::Select;
use crate::query::update::Update;
use crate::types::{Result, Transaction};

#[async_trait]
pub trait DataSource {

    fn insert<'a, T>(&'static self, v: &'a T) -> Insert<'a> where T: Cherry + 'static {
        Insert::insert(self.type_id(),  v)
    }

    fn insert_bulk<'a, T>(&'static self, v: &'a [T]) -> Insert<'a> where T: Cherry + 'static {
        Insert::insert_bulk(self.type_id(), v)
    }

    fn insert_ignore<'a, T>(&'static self, v: &'a [T]) -> Insert<'a> where T: Cherry + 'static {
        Insert::insert_ignore(self.type_id(), v)
    }

    fn insert_replace<'a, T>(&'static self, v: &'a [T]) -> Insert<'a> where T: Cherry + 'static {
        Insert::insert_replace(self.type_id(), v)
    }

    fn insert_update<'a, T>(&'static self, v: &'a [T]) -> InsertUpdate<'a>
        where T: Cherry + 'static {
        InsertUpdate::insert_update(self.type_id(), v)
    }

    fn delete<'a, T>(&'static self) -> Delete<'a> where T: Cherry + 'static {
        Delete::new::<T>(self.type_id())
    }

    fn update<'a, T>(&'static self) -> Update<'a> where T: Cherry + 'static {
        Update::new::<T>(self.type_id())
    }

    fn select<'a, T>(&'static self) -> Select<'a, T> where T: Cherry + 'static {
        Select::new(self.type_id())
    }

    async fn begin<'a>(&'static self) -> Result<Transaction<'a>>  {
        Ok(connection::get(self.type_id())?.begin().await?)
    }

}
