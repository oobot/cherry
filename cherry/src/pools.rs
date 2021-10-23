use std::collections::BTreeMap;
use std::time::Duration;

use anyhow::Error;
use once_cell::sync::OnceCell;
use sqlx::{AnyPool, Database, MssqlPool, MySqlPool, PgPool, Pool, SqlitePool};
use sqlx::pool::PoolOptions;

static POOLS: OnceCell<DBPools> = OnceCell::new();

pub async fn setup_pools(config: Config) -> Result<(), Error> {
    let mut pool = DBPools::default();
    for (k, c) in config.mysql {
        pool.mysql_pool.insert(k, to_pool(&c).await?);
    }
    for (k, c) in config.postgres {
        pool.pg_pool.insert(k, to_pool(&c).await?);
    }
    for (k, c) in config.mssql {
        pool.mssql_pool.insert(k, to_pool(&c).await?);
    }
    for (k, c) in config.sqlite {
        pool.sqlite_pool.insert(k, to_pool(&c).await?);
    }
    for (k, c) in config.any {
        pool.any_pool.insert(k, to_pool(&c).await?);
    }

    POOLS.set(pool).map_err(|_| anyhow!("Failed to set pools."))?;

    Ok(())
}

async fn to_pool<DB: Database>(conf: &PoolConfig) -> Result<Pool<DB>, Error> {
    let pool = PoolOptions::<DB>::new()
        .max_connections(conf.max_connections.unwrap_or(10))
        .min_connections(conf.min_connections.unwrap_or(0))
        .test_before_acquire(conf.test_before_acquire.unwrap_or(true))
        .max_lifetime(Duration::from_secs(conf.max_lifetime.unwrap_or(30 * 60)))
        .idle_timeout(Duration::from_secs(conf.idle_timeout.unwrap_or(10 * 60)))
        .connect_timeout(Duration::from_secs(conf.connect_timeout.unwrap_or(30)))
        .connect(conf.url.as_str()).await?;
    Ok(pool)
}

pub(crate) fn get() -> &'static DBPools {
    POOLS.get().expect("Pools is empty")
}

#[derive(Debug, Default)]
pub(crate) struct DBPools {
    pub mysql_pool: BTreeMap<String, MySqlPool>,
    pub pg_pool: BTreeMap<String, PgPool>,
    pub mssql_pool: BTreeMap<String, MssqlPool>,
    pub sqlite_pool: BTreeMap<String, SqlitePool>,
    pub any_pool: BTreeMap<String, AnyPool>,
}

#[derive(Deserialize)]
pub struct Config {
    pub mysql: BTreeMap<String, PoolConfig>,
    pub postgres: BTreeMap<String, PoolConfig>,
    pub mssql: BTreeMap<String, PoolConfig>,
    pub sqlite: BTreeMap<String, PoolConfig>,
    pub any: BTreeMap<String, PoolConfig>,
}

#[derive(Debug, Deserialize)]
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

// pub fn register_mysql<S: MySqlTemplate<S>, T: MySqlTemplate<S>>(tpl: T) -> Result<(), Error> {
//     todo!()
// }

