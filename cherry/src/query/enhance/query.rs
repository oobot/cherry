use std::any::TypeId;

use sql_builder::SqlBuilder;
use sqlx::encode::Encode;
use sqlx::Type;

use crate::{Cherry, impl_query_where};
use crate::adapt::arguments::Arguments;
use crate::adapt::transaction::Transaction;
use crate::query::TxMode;

pub(crate) struct Query<'a> {
    pub(crate) datasource: TypeId,
    pub(crate) sql_builder: SqlBuilder,
    pub(crate) arguments: Arguments<'a>,
    pub(crate) tx: TxMode<'a>,
}

impl<'a> Query<'a> {
    
    pub(crate) fn new<T: Cherry>(datasource: TypeId, sql_builder: SqlBuilder) -> Self {
        Self { datasource, sql_builder, arguments: Arguments::new(), tx: TxMode::None, }
    }

    pub(crate) fn tx(&mut self, tx: &'a mut Transaction<'a>) -> &mut Self {
        self.tx = TxMode::Manual(tx);
        self
    }

    pub(crate) fn tx_auto(&mut self) -> &mut Self {
        self.tx = TxMode::Auto;
        self
    }

    #[cfg(feature = "mysql")]
    impl_query_where!(sqlx::MySql);
    #[cfg(feature = "postgres")]
    impl_query_where!(sqlx::Postgres);
    #[cfg(feature = "sqlite")]
    impl_query_where!(sqlx::Sqlite);
    #[cfg(feature = "mssql")]
    impl_query_where!(sqlx::Mssql);

}

#[macro_export]
macro_rules! impl_query_where {
    ($db: ty) => {
         pub(crate) fn and_where_eq<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.and_where_eq(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn and_where_ne<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.and_where_ne(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn and_where_ge<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.and_where_ge(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn and_where_le<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.and_where_le(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn and_where_gt<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.and_where_gt(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn and_where_lt<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
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
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.and_where_between(f, '?', '?');
            self.arguments.add(min).add(max);
            self
        }

        pub(crate) fn and_where_not_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.and_where_not_between(f, '?', '?');
            self.arguments.add(min).add(max);
            self
        }

        pub(crate) fn and_where_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + Sync + 'a
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
                V: Encode<'a, $db> + Type<$db> + Send + Sync + 'a
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
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.or_where_eq(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn or_where_ne<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.or_where_ne(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn or_where_ge<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.or_where_ge(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn or_where_le<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.or_where_le(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn or_where_gt<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.or_where_gt(f, '?');
            self.arguments.add(v);
            self
        }

        pub(crate) fn or_where_lt<S, V>(&mut self, f: S, v: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
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
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.or_where_between(f, '?', '?');
            self.arguments.add(min).add(max);
            self
        }

        pub(crate) fn or_where_not_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.sql_builder.or_where_not_between(f, '?', '?');
            self.arguments.add(min).add(max);
            self
        }

        pub(crate) fn or_where_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + Sync + 'a
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
                V: Encode<'a, $db> + Type<$db> + Send + Sync + 'a
        {
            self.sql_builder.or_where_not_in(f, &vec!["?"; v.len()]);
            v.iter().for_each(|v| {
                self.arguments.add(v);
            });
            self
        }
    }
}

/*
impl<'a> Query<'a>{

    pub(crate) fn and_where_eq<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.and_where_eq(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_ne<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.and_where_ne(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_ge<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.and_where_ge(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_le<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.and_where_le(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_gt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.and_where_gt(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn and_where_lt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
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
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.and_where_between(f, '?', '?');
        self.arguments.add(min).add(max);
        self
    }

    pub(crate) fn and_where_not_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.and_where_not_between(f, '?', '?');
        self.arguments.add(min).add(max);
        self
    }

    pub(crate) fn and_where_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
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

    pub(crate) fn and_where_not_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
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

    pub(crate) fn or_where_eq<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.or_where_eq(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_ne<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.or_where_ne(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_ge<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.or_where_ge(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_le<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.or_where_le(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_gt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.or_where_gt(f, '?');
        self.arguments.add(v);
        self
    }

    pub(crate) fn or_where_lt<S, V>(&mut self, f: S, v: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
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
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.or_where_between(f, '?', '?');
        self.arguments.add(min).add(max);
        self
    }

    pub(crate) fn or_where_not_between<S, V>(&mut self, f: S, min: V, max: V) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + 'a
    {
        self.sql_builder.or_where_not_between(f, '?', '?');
        self.arguments.add(min).add(max);
        self
    }

    pub(crate) fn or_where_in<S, V>(&mut self, f: S, v: &'a [V]) -> &mut Self
        where
            S: ToString,
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + Sync + 'a
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
            V: Encode<'a, sqlx::mysql::MySql> + Type<sqlx::mysql::MySql> + Send + Sync + 'a
    {
        self.sql_builder.or_where_not_in(f, &vec!["?"; v.len()]);
        v.iter().for_each(|v| {
            self.arguments.add(v);
        });
        self
    }

}
*/