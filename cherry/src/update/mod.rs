use std::any::TypeId;
use std::fmt::Display;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::types::Type;

use crate::{Cherry, connection};
use crate::adapt::arguments::Arguments;
use crate::adapt::query_result::QueryResult;
use crate::adapt::transaction::Transaction;
use crate::execute::{self, Data, TxMode};
use crate::Result;

pub struct Update<'a> {
    datasource: TypeId,
    sql_builder: SqlBuilder,
    arguments: Arguments<'a>,
    tx: TxMode<'a>,
}

impl<'a> Update<'a> {

    pub(crate) fn new<T: Cherry>(datasource: TypeId) -> Self {
        Self {
            datasource,
            sql_builder: SqlBuilder::update_table(T::table()),
            arguments: Arguments::new(),
            tx: TxMode::None,
        }
    }

    pub fn tx(mut self, tx: &'a mut Transaction<'a>) -> Self {
        self.tx_ref(tx);
        self
    }

    fn tx_ref(&mut self, tx: &'a mut Transaction<'a>) -> &Self {
        self.tx = TxMode::Manual(tx);
        self
    }

    pub fn tx_auto(mut self) -> Self {
        self.tx_auto_ref();
        self
    }

    fn tx_auto_ref(&mut self) -> &Self {
        self.tx = TxMode::Auto;
        self
    }

    pub async fn execute(self) -> Result<QueryResult>  {
        let data = Data {
            datasource: self.datasource,
            sql: self.sql_builder.sql()?,
            arguments: self.arguments,
            tx: self.tx
        };
        execute::execute(data).await
    }

}

impl<'a> Update<'a>{
    pub fn set<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.set_ref(f, v);
        self
    }

    pub fn set_ref<S, V>(&mut self, f: S, v: V) -> &Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.set(f, '?');
        self.arguments.add(v);
        self
    }

}

impl<'a> Update<'a>{

    pub fn and_where_eq<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_eq(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn and_where_ne<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_ne(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn and_where_ge<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_ge(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn and_where_le<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_le(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn and_where_gt<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_gt(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn and_where_lt<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_lt(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn and_where_is_null<S, V>(mut self, f: S) -> Self where S: ToString {
        self.sql_builder.and_where_is_null(f);
        self
    }

    pub fn and_where_is_not_null<S, V>(mut self, f: S) -> Self where S: ToString {
        self.sql_builder.and_where_is_not_null(f);
        self
    }

    pub fn and_where_between<S, V>(mut self, f: S, min: V, max: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_between(f, '?', '?');
        self.arguments.add(min).add(max);
        self
    }

    pub fn and_where_not_between<S, V>(mut self, f: S, min: V, max: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_not_between(f, '?', '?');
        self.arguments.add(min).add(max);
        self
    }

    pub fn and_where_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + Sync + 'a 
    {
        self.sql_builder.and_where_in(f, &vec!["?"; v.len()]);
        v.iter().for_each(|v| {
            self.arguments.add(v);
        });
        self
    }

    pub fn and_where_not_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + Sync + 'a 
    {
        self.sql_builder.and_where_not_in(f, &vec!["?"; v.len()]);
        v.iter().for_each(|v| {
            self.arguments.add(v);
        });
        self
    }

    // ***********************************************************************

    pub fn or_where_eq<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_eq(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn or_where_ne<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_ne(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn or_where_ge<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_ge(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn or_where_le<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_le(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn or_where_gt<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_gt(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn or_where_lt<S, V>(mut self, f: S, v: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_lt(f, '?');
        self.arguments.add(v);
        self
    }

    pub fn or_where_is_null<S, V>(mut self, f: S) -> Self where S: ToString {
        self.sql_builder.and_where_is_null(f);
        self
    }

    pub fn or_where_is_not_null<S, V>(mut self, f: S) -> Self where S: ToString {
        self.sql_builder.and_where_is_not_null(f);
        self
    }

    pub fn or_where_between<S, V>(mut self, f: S, min: V, max: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_between(f, '?', '?');
        self.arguments.add(min).add(max);
        self
    }

    pub fn or_where_not_between<S, V>(mut self, f: S, min: V, max: V) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a 
    {
        self.sql_builder.and_where_not_between(f, '?', '?');
        self.arguments.add(min).add(max);
        self
    }

    pub fn or_where_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + Sync + 'a 
    {
        self.sql_builder.and_where_in(f, &vec!["?"; v.len()]);
        v.iter().for_each(|v| {
            self.arguments.add(v);
        });
        self
    }

    pub fn or_where_not_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + Sync + 'a 
    {
        self.sql_builder.and_where_not_in(f, &vec!["?"; v.len()]);
        v.iter().for_each(|v| {
            self.arguments.add(v);
        });
        self
    }

}

fn test() {
    // let a = Update {
    //     sql_builder: SqlBuilder::update_table(""),
    //     arguments: Arguments::new()
    // };
    //
    // let f = "ag".to_string();
    // a.and_where_eq(&f, "sg");

}


/*
macro_rules! gen_where {
    ($t: ty) => {
        impl $t {
        }
    }
}
*/