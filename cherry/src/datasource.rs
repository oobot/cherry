use crate::gen_pool;

pub trait DataSource {
    type Database: sqlx::Database;
}

#[cfg(feature = "mysql")]
gen_pool!(sqlx::MySql);
#[cfg(feature = "postgres")]
gen_pool!(sqlx::Postgres);
#[cfg(feature = "sqlite")]
gen_pool!(sqlx::Sqlite);
#[cfg(feature = "mssql")]
gen_pool!(sqlx::Mssql);

#[macro_export]
macro_rules! gen_pool {
    ($db: ty) => {
        use std::any::{Any, TypeId};
        use std::collections::BTreeMap;

        use anyhow::anyhow;
        use once_cell::sync::OnceCell;
        use sqlx::Pool;

        use crate::config::PoolConfig;
        use crate::Result;

        static POOLS: OnceCell<BTreeMap<TypeId, Pool<$db>>> = OnceCell::new();

        #[allow(dead_code)]
        pub(crate) fn get<T: 'static>() -> Result<&'static Pool<$db>> {
            let type_id = TypeId::of::<T>();
            let value = POOLS.get()
                .ok_or_else(|| anyhow!("Pools is empty."))?
                .get(&type_id)
                .ok_or_else(|| anyhow!("No pool found for key: {:?}", type_id))?;
            Ok(value)
        }

        pub async fn set(config: BTreeMap<Box<dyn DataSource<Database=$db>>, PoolConfig>)
            -> Result<()> {
            let mut pools = BTreeMap::new();
            for (k, v) in config {
                pools.insert((&*k).type_id(), v.to_pool().await?);
            }

            POOLS.set(pools).map_err(|_| anyhow!("Failed to set pools."))?;
            Ok(())
        }
    }
}
