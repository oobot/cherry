use anyhow::Error;
use sqlx::{Pool, Sqlite};

pub struct SqlitePool {
    pub(crate) inner: Pool<Sqlite>,
}

impl SqlitePool {
    pub async fn connect(url: &str) -> Result<Self, Error> {
        Ok(Self { inner: Pool::<Sqlite>::connect(url).await? })
    }
}