#[macro_export]
macro_rules! impl_tx {
    () => {
        pub fn tx(mut self, tx: &'a mut Transaction<'a>) -> Self {
            self.query.tx(tx);
            self
        }

        pub fn tx_ref(&mut self, tx: &'a mut Transaction<'a>) -> &Self {
            self.query.tx(tx);
            self
        }

        pub fn tx_auto(mut self) -> Self {
            self.query.tx_auto();
            self
        }

        pub fn tx_auto_ref(&mut self) -> &Self {
            self.query.tx_auto();
            self
        }
    }
}

#[macro_export]
macro_rules! impl_where {
    ($db: ty) => {
        pub fn and_where_eq<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.and_where_eq(f, v);
            self
        }

        pub fn and_where_ne<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.and_where_ne(f, v);
            self
        }

        pub fn and_where_ge<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.and_where_ge(f, v);
            self
        }

        pub fn and_where_le<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.and_where_le(f, v);
            self
        }

        pub fn and_where_gt<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.and_where_gt(f, v);
            self
        }

        pub fn and_where_lt<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.and_where_lt(f, v);
            self
        }

        pub fn and_where_is_null<S>(mut self, f: S) -> Self where S: ToString {
            self.query.and_where_is_null(f);
            self
        }

        pub fn and_where_is_not_null<S>(mut self, f: S) -> Self where S: ToString {
            self.query.and_where_is_not_null(f);
            self
        }

        pub fn and_where_between<S, V>(mut self, f: S, min: V, max: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.and_where_between(f, min, max);
            self
        }

        pub fn and_where_not_between<S, V>(mut self, f: S, min: V, max: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.and_where_not_between(f, min, max);
            self
        }

        pub fn and_where_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + Sync + 'a
        {
            self.query.and_where_in(f, v);
            self
        }

        pub fn and_where_not_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + Sync + 'a
        {
            self.query.and_where_not_in(f, v);
            self
        }

        // ***********************************************************************

        pub fn or_where_eq<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.or_where_eq(f, v);
            self
        }

        pub fn or_where_ne<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.or_where_ne(f, v);
            self
        }

        pub fn or_where_ge<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.or_where_ge(f, v);
            self
        }

        pub fn or_where_le<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.or_where_le(f, v);
            self
        }

        pub fn or_where_gt<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.or_where_gt(f, v);
            self
        }

        pub fn or_where_lt<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.or_where_lt(f, v);
            self
        }

        pub fn or_where_is_null<S>(mut self, f: S) -> Self where S: ToString {
            self.query.or_where_is_null(f);
            self
        }

        pub fn or_where_is_not_null<S, V>(mut self, f: S) -> Self where S: ToString {
            self.query.or_where_is_not_null(f);
            self
        }

        pub fn or_where_between<S, V>(mut self, f: S, min: V, max: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.or_where_between(f, min, max);
            self
        }

        pub fn or_where_not_between<S, V>(mut self, f: S, min: V, max: V) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + 'a
        {
            self.query.or_where_not_between(f, min, max);
            self
        }

        pub fn or_where_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + Sync + 'a
        {
            self.query.or_where_in(f, v);
            self
        }

        pub fn or_where_not_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
            where
                S: ToString,
                V: Encode<'a, $db> + Type<$db> + Send + Sync + 'a
        {
            self.query.or_where_not_in(f, v);
            self
        }
    }
}
