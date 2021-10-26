use std::collections::BTreeMap;

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use sqlx::{Database, Mssql, MySql, Pool, Postgres, Sqlite};

use crate::Result;
use crate::config::{Config, PoolConfig};

static POOLS: OnceCell<DBPools> = OnceCell::new();

pub async fn setup_pools(config: Config) -> Result<()> {
    let mut pool = DBPools::default();
    set_pool::<MySql>(config.mysql, &mut pool.mysql_pool).await?;
    set_pool::<Postgres>(config.postgres, &mut pool.pg_pool).await?;
    set_pool::<Sqlite>(config.sqlite, &mut pool.sqlite_pool).await?;
    set_pool::<Mssql>(config.mssql, &mut pool.mssql_pool).await?;

    Ok(POOLS.set(pool).map_err(|_| anyhow!("Failed to set pools."))?)
}

async fn set_pool<DB: Database>(src: Option<BTreeMap<String, PoolConfig>>,
                                target: &mut BTreeMap<String, Pool<DB>>)
    -> Result<()> {
    if let Some(values) = src {
        for (k, v) in values {
            target.insert(k, v.to_pool().await?);
        }
    }
    Ok(())
}

pub(crate) fn get() -> &'static DBPools {
    POOLS.get().expect("Pools is empty")
}

#[derive(Debug, Default)]
pub(crate) struct DBPools {
    pub mysql_pool: BTreeMap<String, Pool<MySql>>,
    pub pg_pool: BTreeMap<String, Pool<Postgres>>,
    pub mssql_pool: BTreeMap<String, Pool<Mssql>>,
    pub sqlite_pool: BTreeMap<String, Pool<Sqlite>>,
}
