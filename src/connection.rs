use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use anyhow::anyhow;
use once_cell::sync::{Lazy, OnceCell};
use sqlx::pool::PoolOptions;

use crate::types::{Pool, Result};

static CONFIG: Lazy<Mutex<Vec<(String, Pool)>>> = Lazy::new(|| Mutex::new(vec![]));
static POOLS: OnceCell<HashMap<String, Pool>> = OnceCell::new();

pub(crate) async fn add_conn(ds: &str, conn: PoolConfig) -> Result<()> {
    if POOLS.get().is_some() {
        Err(anyhow!("Connection pools must be set up before use"))?;
    }

    let pool = conn.to_pool().await?;
/*
    if conn.test_before_acquire.unwrap_or(false) {
        let sql = if cfg!(feature = "mysql") || cfg!(feature = "postgres") {
            "select version()"
        } else if cfg!(feature = "sqlite") {
            "select sqlite_version()"
        } else { // cfg!(feature = "mssql")
            "select @@version"
        };

        let row: (String,) = sqlx::query_as(sql).fetch_one(&pool).await?;
        info!("Connect to database successfully, version {}", row.0);
    }
*/
    CONFIG.lock().map_err(|e| anyhow!("{}", e))?
        .push((ds.to_string(), pool));

    Ok(())
}

pub(crate) fn get(ds: &str) -> Result<&Pool> {
    if POOLS.get().is_none() {
        let mut map = HashMap::new();
        let mut lock = CONFIG.lock().map_err(|e| anyhow!("{}", e))?;
        while let Some((k, v)) = lock.pop() {
            map.insert(k, v);
        }
        POOLS.set(map).expect("initialize connection pools multiple times");
    }

    match POOLS.get() {
        Some(pools) => match pools.get(ds) {
            Some(pool) => Ok(pool),
            _ => Err(anyhow!("the pool {} not exists", ds))?,
        }
        _ => Err(anyhow!("must be initialized before using connection pools"))?
    }
}

#[cfg_attr(feature = "json", derive(serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct PoolConfig {
    pub url: String,
    pub test_before_acquire: Option<bool>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub acquire_timeout: Option<u64>,
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


/*static POOLS: OnceCell<BTreeMap<TypeId, Pool>> = OnceCell::new();

pub async fn setup_pools<T>(config: T) -> Result<()>
    where T: IntoIterator<Item = (TypeId, PoolConfig)> {
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
}*/