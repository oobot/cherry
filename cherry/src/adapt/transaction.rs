macro_rules! gen_tx {
    ($db: ty) => {
        pub struct Transaction<'t> {
            pub(crate) inner: sqlx::Transaction<'t, $db>,
        }
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
