use sqlx::{Encode, Postgres, Type};

pub struct PgArguments(pub(crate) sqlx::postgres::PgArguments);

impl PgArguments {
    pub fn new() -> Self {
        Self(sqlx::postgres::PgArguments::default())
    }

    pub fn add<'a, T: Encode<'a, Postgres> + Type<Postgres>>(&mut self, v: T) -> &mut Self {
        sqlx::Arguments::add(&mut self.0, v);
        // use sqlx::Arguments;
        // self.0.add(v);
        self
    }
}