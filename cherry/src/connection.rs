use std::any::TypeId;
use std::collections::BTreeMap;
use std::time::Duration;

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use sqlx::pool::PoolOptions;

use crate::types::{Pool, Result};

static POOLS: OnceCell<BTreeMap<TypeId, Pool>> = OnceCell::new();

pub async fn setup_pools(config: BTreeMap<TypeId, PoolConfig>) -> Result<()> {
    let mut pools = BTreeMap::new();
    for (key, v) in config {
        pools.insert(key, v.to_pool().await?);
    }

    POOLS.set(pools).map_err(|_| anyhow!("Failed to set pools."))?;
    Ok(())
}

pub(crate) fn get(type_id: TypeId) -> Result<&'static Pool> {
    let value = POOLS.get()
        .ok_or_else(|| anyhow!("Pools is empty."))?
        .get(&type_id)
        .ok_or_else(|| anyhow!("No pool found for key: {:?}", type_id))?;
    Ok(value)
}


#[cfg_attr(feature = "json", derive(serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct PoolConfig {
    pub url: String,
    pub test_before_acquire: Option<bool>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout: Option<u64>,
    pub max_lifetime: Option<u64>,
    pub idle_timeout: Option<u64>,
    // after_connect: None,
    // before_acquire: None,
    // after_release: None,
    // fair: Option<bool>,
}

impl PoolConfig {

    pub(crate) async fn to_pool(&self) -> Result<Pool> {
        let mut opts = PoolOptions::new();
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

        Ok( opts.connect(self.url.as_str()).await? )
    }
}
