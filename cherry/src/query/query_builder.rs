use std::any::TypeId;
use std::marker::PhantomData;

use sql_builder::SqlBuilder;
use sqlx::{Arguments as SqlxArguments, Type};
use sqlx::encode::Encode;

use crate::Cherry;
use crate::types::{Arguments, Database};

pub(crate) struct QueryBuilder<'a> {
    _keep: PhantomData<&'a ()>,
    pub(crate) datasource: TypeId,
    pub(crate) sql_builder: SqlBuilder,
    pub(crate) arguments: Arguments<'a>,
}

impl<'a> QueryBuilder<'a> {
    
    pub(crate) fn new<T: Cherry>(datasource: TypeId, sql_builder: SqlBuilder) -> Self {
        Self { _keep: PhantomData, datasource, sql_builder, arguments: Arguments::default() }
    }

    pub(crate) fn add_arguments<V>(&mut self, v: V) -> &mut Self
        where V: Encode<'a, Database> + Type<Database> + Send + 'a {
        self.arguments.add(v);
        self
    }

}

impl<'a> QueryBuilder<'a>{

    pub(crate) fn and_where_eq<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.and_where_eq(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_ne<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.and_where_ne(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_ge<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.and_where_ge(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_le<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.and_where_le(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_gt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.and_where_gt(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_lt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.and_where_lt(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_is_null<S>(&mut self, f: S) -> &mut Self where S: ToString {
        self.sql_builder.and_where_is_null(f);
        self
    }

    pub(crate) fn and_where_is_not_null<S>(&mut self, f: S) -> &mut Self where S: ToString {
        self.sql_builder.and_where_is_not_null(f);
        self
    }

    pub(crate) fn and_where_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.and_where_between(f, '?', '?');
        self.arguments.add(min);
        self.arguments.add(max);
        self
    }

    pub(crate) fn and_where_not_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.and_where_not_between(f, '?', '?');
        self.arguments.add(min);
        self.arguments.add(max);
        self
    }

    pub(crate) fn and_where_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a
    {
        self.sql_builder.and_where_in(f, &vec!["?"; v.len()]);
        v.iter().for_each(|v| {
            self.arguments.add(v);
        });
        self
    }

    pub(crate) fn and_where_not_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a
    {
        self.sql_builder.and_where_not_in(f, &vec!["?"; v.len()]);
        v.iter().for_each(|v| {
            self.arguments.add(v);
        });
        self
    }

    // ***********************************************************************

    pub(crate) fn or_where_eq<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.or_where_eq(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_ne<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.or_where_ne(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_ge<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.or_where_ge(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_le<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.or_where_le(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_gt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.or_where_gt(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_lt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.or_where_lt(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_is_null<S>(&mut self, f: S) -> &mut Self where S: ToString {
        self.sql_builder.or_where_is_null(f);
        self
    }

    pub(crate) fn or_where_is_not_null<S>(&mut self, f: S) -> &mut Self where S: ToString {
        self.sql_builder.or_where_is_not_null(f);
        self
    }

    pub(crate) fn or_where_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.or_where_between(f, '?', '?');
        self.arguments.add(min);
        self.arguments.add(max);
        self
    }

    pub(crate) fn or_where_not_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + 'a
    {
        self.sql_builder.or_where_not_between(f, '?', '?');
        self.arguments.add(min);
        self.arguments.add(max);
        self
    }

    pub(crate) fn or_where_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a
    {
        self.sql_builder.or_where_in(f, &vec!["?"; v.len()]);
        v.iter().for_each(|v| {
            self.arguments.add(v);
        });
        self
    }

    pub(crate) fn or_where_not_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a
    {
        self.sql_builder.or_where_not_in(f, &vec!["?"; v.len()]);
        v.iter().for_each(|v| {
            self.arguments.add(v);
        });
        self
    }

}
