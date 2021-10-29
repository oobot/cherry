use std::marker::PhantomData;

use sqlx::Type;
use sqlx::encode::Encode;

macro_rules! gen_arguments {
    ($db: ty, $arg: ty) => {
        pub struct Arguments<'a> {
            _keep: PhantomData<&'a ()>,
            pub(crate) inner: $arg,
            pub(crate) count: usize,
        }

        impl<'a> Arguments<'a> {
            pub fn new() -> Self {
                Self {
                    _keep: Default::default(),
                    inner: <$db as sqlx::database::HasArguments>::Arguments::default(),
                    count: 0,
                }
            }

            pub fn from<T>(value: T) -> Self
                where T: 'a + Send + Encode<'a, $db> + Type<$db> {
                let mut arg = Self::new();
                arg.add(value);
                arg
            }

            pub fn add<T>(&mut self, value: T) -> &mut Self
                where T: 'a + Send + Encode<'a, $db> + Type<$db> {
                sqlx::Arguments::add(&mut self.inner, value);
                self.count += 1;
                self
            }
        }

    }
}

#[cfg(feature = "mysql")]
gen_arguments!(sqlx::MySql, sqlx::mysql::MySqlArguments);
#[cfg(feature = "postgres")]
gen_arguments!(sqlx::Postgres, sqlx::postgres::PgArguments);
#[cfg(feature = "sqlite")]
gen_arguments!(sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>);
#[cfg(feature = "mssql")]
gen_arguments!(sqlx::Mssql, sqlx::mssql::MssqlArguments);
