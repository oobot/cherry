use anyhow::Error;
use sqlx::{Pool, Postgres};

pub struct PgPool {
    pub(crate) inner: Pool<Postgres>,
}

impl PgPool {
    pub async fn connect(url: &str) -> Result<Self, Error> {
        Ok(Self { inner: Pool::<Postgres>::connect(url).await? })
    }
}