#[macro_export]
macro_rules! gen_execute {
    () => {
        pub async fn execute(mut self) -> Result<QueryResult> {
            let pool = connection::get(self.query.datasource)?;
            let result = sqlx::query_with(self.build_sql()?.as_str(), self.query.arguments)
                .execute(pool).await?;
            Ok(result)
        }

        pub async fn execute_tx(mut self) -> Result<QueryResult> {
            let mut tx = connection::get(self.query.datasource)?.begin().await?;
            let result = sqlx::query_with(self.build_sql()?.as_str(), self.query.arguments)
                .execute(&mut tx).await?;
            tx.commit().await?;
            Ok(result)
        }

        pub async fn execute_with(mut self, tx: &mut Transaction<'a>) -> Result<QueryResult>  {
            let result = sqlx::query_with(self.build_sql()?.as_str(), self.query.arguments)
                .execute(tx).await?;
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! gen_where {
    () => {
        pub fn and_where_eq<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.and_where_eq(f, v);
            self
        }

        pub fn and_where_ne<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.and_where_ne(f, v);
            self
        }

        pub fn and_where_ge<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.and_where_ge(f, v);
            self
        }

        pub fn and_where_le<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.and_where_le(f, v);
            self
        }

        pub fn and_where_gt<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.and_where_gt(f, v);
            self
        }

        pub fn and_where_lt<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
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
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.and_where_between(f, min, max);
            self
        }

        pub fn and_where_not_between<S, V>(mut self, f: S, min: V, max: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.and_where_not_between(f, min, max);
            self
        }

        pub fn and_where_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a
        {
            self.query.and_where_in(f, v);
            self
        }

        pub fn and_where_not_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a
        {
            self.query.and_where_not_in(f, v);
            self
        }

        // ***********************************************************************

        pub fn or_where_eq<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.or_where_eq(f, v);
            self
        }

        pub fn or_where_ne<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.or_where_ne(f, v);
            self
        }

        pub fn or_where_ge<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.or_where_ge(f, v);
            self
        }

        pub fn or_where_le<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.or_where_le(f, v);
            self
        }

        pub fn or_where_gt<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.or_where_gt(f, v);
            self
        }

        pub fn or_where_lt<S, V>(mut self, f: S, v: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
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
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.or_where_between(f, min, max);
            self
        }

        pub fn or_where_not_between<S, V>(mut self, f: S, min: V, max: V) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + 'a
        {
            self.query.or_where_not_between(f, min, max);
            self
        }

        pub fn or_where_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a
        {
            self.query.or_where_in(f, v);
            self
        }

        pub fn or_where_not_in<S, V>(mut self, f: S, v: &'a [V]) -> Self
            where
                S: ToString,
                V: Encode<'a, Database> + Type<Database> + Send + Sync + 'a
        {
            self.query.or_where_not_in(f, v);
            self
        }
    }
}
