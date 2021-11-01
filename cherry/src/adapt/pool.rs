macro_rules! gen_pool {
    ($db: ty) => {
        pub(crate) struct Pool {
            pub(crate) inner: sqlx::Pool<$db>
        }
    }
}

#[cfg(feature = "mysql")]
gen_pool!(sqlx::MySql);
#[cfg(feature = "postgres")]
gen_pool!(sqlx::Postgres);
#[cfg(feature = "sqlite")]
gen_pool!(sqlx::Sqlite);
#[cfg(feature = "mssql")]
gen_pool!(sqlx::Mssql);
