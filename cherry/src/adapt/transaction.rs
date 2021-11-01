use std::any::TypeId;

use crate::{connection, Result};

macro_rules! gen_tx {
    ($db: ty) => {
        pub struct Transaction<'t> {
            pub(crate) inner: sqlx::Transaction<'t, $db>,
        }
    }
}

impl<'t> Transaction<'t> {

    pub async fn begin(datasource: TypeId) -> Result<Transaction<'t>> {
        let tx = Transaction {
            inner: connection::get(datasource)?.inner.begin().await?,
        };
        Ok(tx)
    }

    pub async fn commit(mut self) -> Result<()> {
        Ok(self.inner.commit().await?)
    }

    pub async fn rollback(mut self) -> Result<()> {
        Ok(self.inner.rollback().await?)
    }

}


#[cfg(feature = "mysql")]
gen_tx!(sqlx::MySql);
#[cfg(feature = "postgres")]
gen_tx!(sqlx::Postgres);
#[cfg(feature = "sqlite")]
gen_tx!(sqlx::Sqlite);
#[cfg(feature = "mssql")]
gen_tx!(sqlx::Mssql);
