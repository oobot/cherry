use std::collections::BTreeMap;
use std::time::Duration;

use anyhow::Error;
use once_cell::sync::OnceCell;
use sqlx::{AnyPool, Database, MssqlPool, MySqlPool, PgPool, Pool, SqlitePool};
use sqlx::pool::PoolOptions;

use crate::config::Config;

static POOLS: OnceCell<DBPools> = OnceCell::new();

pub async fn setup_pools(config: Config) -> Result<(), Error> {
    let mut pool = DBPools::default();
    for (k, c) in config.mysql {
        pool.mysql_pool.insert(k, c.to_pool().await?);
    }
    for (k, c) in config.postgres {
        pool.pg_pool.insert(k, c.to_pool().await?);
    }
    for (k, c) in config.mssql {
        pool.mssql_pool.insert(k, c.to_pool().await?);
    }
    for (k, c) in config.sqlite {
        pool.sqlite_pool.insert(k, c.to_pool().await?);
    }
    for (k, c) in config.any {
        pool.any_pool.insert(k, c.to_pool().await?);
    }

    Ok(POOLS.set(pool).map_err(|_| anyhow!("Failed to set pools."))?)
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
