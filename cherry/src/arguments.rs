use sqlx::{Database, Type};
use sqlx::database::HasArguments;
use sqlx::encode::Encode;

pub struct Arguments<'q, DB: Database> {
    pub(crate) inner: <DB as HasArguments<'q>>::Arguments,
    pub(crate) count: usize,
}

impl<'q, DB: Database> Arguments<'q, DB> {
    pub fn new() -> Self {
        Self {
            inner: <DB as HasArguments<'q>>::Arguments::default(),
            count: 0,
        }
    }

    pub fn from<T>(value: T) -> Self
        where T: 'q + Send + Encode<'q, DB> + Type<DB> {
        let mut arg = Self::new();
        arg.add(value);
        arg
    }

    pub fn add<T>(&mut self, value: T) -> &mut Self
        where T: 'q + Send + Encode<'q, DB> + Type<DB> {
        sqlx::Arguments::add(&mut self.inner, value);
        // self.inner.add(value);
        self.count += 1;
        self
    }
}
