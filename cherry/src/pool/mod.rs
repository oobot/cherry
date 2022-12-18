use std::time::Duration;

use sqlx::{Database, Error, Pool};

#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg_attr(feature = "json", derive(serde::Deserialize))]
#[derive(Default, Clone)]
pub struct PoolOptions {
    pub url: String,
    pub test_before_acquire: Option<bool>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub acquire_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
    pub idle_timeout: Option<u64>,
    // pub fair: Option<bool>,
}

impl PoolOptions  {

    pub(crate) async fn to_pool<DB>(&self) -> Result<Pool<DB>, Error> where DB: Database {
        let mut opts = sqlx::pool::PoolOptions::new();
        if let Some(v) = self.test_before_acquire {
            opts = opts.test_before_acquire(v);
        }
        if let Some(v) = self.max_connections {
            opts = opts.max_connections(v);
        }
        if let Some(v) = self.min_connections {
            opts = opts.min_connections(v);
        }
        if let Some(v) = self.acquire_timeout {
            opts = opts.acquire_timeout(Duration::from_secs(v));
        }
        if let Some(v) = self.max_lifetime {
            opts = opts.max_lifetime(Duration::from_secs(v));
        }
        if let Some(v) = self.idle_timeout {
            opts = opts.idle_timeout(Duration::from_secs(v));
        }

        Ok( opts.connect(self.url.as_str()).await? )
    }
}
