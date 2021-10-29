use std::any::TypeId;

use crate::Cherry;
use crate::insert::{Insert, InsertUpdate};

pub trait DataSource {

    fn insert<'a, T>(&'a self, v: &'a T) -> Insert<'a> where T: Cherry + 'static {
        Insert::insert(TypeId::of::<T>(),  v)
    }

    fn insert_bulk<'a, T>(&'a self, v: &'a [T]) -> Insert<'a> where T: Cherry + 'static {
        Insert::insert_bulk(TypeId::of::<T>(), v)
    }

    fn insert_ignore<'a, T>(&'a self, v: &'a [T]) -> Insert<'a> where T: Cherry + 'static {
        Insert::insert_ignore(TypeId::of::<T>(), v)
    }

    fn insert_replace<'a, T>(&'a self, v: &'a [T]) -> Insert<'a> where T: Cherry + 'static {
        Insert::insert_replace(TypeId::of::<T>(), v)
    }

    fn insert_update<'a, T>(&'a self, v: &'a [T]) -> InsertUpdate<'a> where T: Cherry + 'static {
        InsertUpdate::insert_update(TypeId::of::<T>(), v)
    }

}
