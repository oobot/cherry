use anyhow::Error;
use sqlx::{MySql, Pool};

pub struct MySqlPool {
    pub(crate) inner: Pool<MySql>,
}

impl MySqlPool {
    pub async fn connect(url: &str) -> Result<Self, Error> {
        Ok(Self { inner: Pool::<MySql>::connect(url).await? })
    }
}