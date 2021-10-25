use std::collections::BTreeMap;
use std::time::Duration;

use serde::Deserialize;
use sqlx::{Database, Pool};
use sqlx::pool::PoolOptions;

use crate::Result;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub mysql: Option<BTreeMap<String, PoolConfig>>,
    pub postgres: Option<BTreeMap<String, PoolConfig>>,
    pub sqlite: Option<BTreeMap<String, PoolConfig>>,
    pub mssql: Option<BTreeMap<String, PoolConfig>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct PoolConfig {
    pub url: String,
    // after_connect: None,
    // before_acquire: None,
    // after_release: None,
    // fair: Option<bool>,
    pub test_before_acquire: Option<bool>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
    pub idle_timeout: Option<u64>,
}

impl PoolConfig {
    pub(crate) async fn to_pool<DB: Database>(&self) -> Result<Pool<DB>> {
        let mut opts = PoolOptions::<DB>::new();
        if let Some(v) = self.test_before_acquire {
            opts = opts.test_before_acquire(v);
        }
        if let Some(v) = self.max_connections {
            opts = opts.max_connections(v);
        }
        if let Some(v) = self.min_connections {
            opts = opts.min_connections(v);
        }
        if let Some(v) = self.connect_timeout {
            opts = opts.connect_timeout(Duration::from_secs(v));
        }
        if let Some(v) = self.max_lifetime {
            opts = opts.max_lifetime(Duration::from_secs(v));
        }
        if let Some(v) = self.idle_timeout {
            opts = opts.idle_timeout(Duration::from_secs(v));
        }

        Ok(opts.connect(self.url.as_str()).await?)
    }
}
