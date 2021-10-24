use anyhow::Error;
use async_trait::async_trait;
use sql_builder::SqlBuilder;
use sqlx::{MySql, Transaction};

use crate::{WrapRows, MySqlArguments};
use crate::Cherry;
use crate::mysql;
use crate::mysql::pool;

#[async_trait]
pub trait MySqlTemplate {

    async fn insert<T>(&self, t: &T) -> Result<u64, Error> where T: Cherry + Sync + Send {
        let sql = mysql::bulk_sql::<T>(1).sql()?;
        let arguments = t.to_arguments().unwrap_mysql()?.inner;
        let x = sqlx::query_with(sql.as_str(), arguments)
            .execute(mysql::pool(Self::key())?)
            .await?;
        Ok(x.rows_affected())
    }

    async fn insert_multi<'a, T>(&self, v: &'a [T], tx: Option<&mut Transaction<'a, MySql>>)
        -> Result<u64, Error>
        where T: Cherry + Sync {
        assert!(v.len() > 0);
        let sql = mysql::bulk_sql::<T>(v.len()).sql()?;
        let arguments = mysql::set_arguments(v)?;
        let x = mysql::execute(Self::key(), sql, arguments, tx).await?;
        Ok(x.rows_affected())
    }

    async fn insert_replace<'a, T>(&self, v: &'a [T], tx: Option<&mut Transaction<'a, MySql>>)
                           -> Result<u64, Error>
        where T: Cherry + Sync {
        assert!(v.len() > 0);
        let sql = mysql::bulk_sql::<T>(v.len()).sql()?
            .replacen("INSERT INTO", "REPLACE INTO", 1);
        let arguments = mysql::set_arguments(v)?;
        let x = mysql::execute(Self::key(), sql, arguments, tx).await?;
        Ok(x.rows_affected())
    }

    async fn insert_ignore<'a, T>(&self, v: &'a [T], tx: Option<&mut Transaction<'a, MySql>>)
        -> Result<u64, Error>
        where T: Cherry + Sync {
        assert!(v.len() > 0);
        let sql = mysql::bulk_sql::<T>(v.len()).sql()?
            .replace("INSERT", "INSERT IGNORE");
        let arguments = mysql::set_arguments(v)?;
        let x = mysql::execute(Self::key(), sql, arguments, tx).await?;
        Ok(x.rows_affected())
    }

    async fn insert_update<'a, T>(
        &self, v: &'a [T], fields: &[&str], tx: Option<&mut Transaction<'a, MySql>>)
        -> Result<u64, Error>
        where T: Cherry + Sync {
        assert!(v.len() > 0 && fields.len() > 0);

        let insert = mysql::bulk_sql::<T>(v.len()).sql()?
            .strip_suffix(";").ok_or(anyhow!("Bad sql"))?
            .to_owned();

        let update = fields.iter().map(|x| format!("{0} = new.{0}, ", x))
            .collect::<String>()
            .strip_suffix(",").ok_or(anyhow!("Bad sql"))?
            .to_owned();

        let sql = format!("{} AS new ON DUPLICATE KEY UPDATE {};", insert, update);

        let arguments = mysql::set_arguments(v)?;
        let x = mysql::execute(Self::key(), sql, arguments, tx).await?;
        Ok(x.rows_affected())
    }

    async fn select<'a, T>(&self, eq_field: &str, args: MySqlArguments<'a>)
                           -> Result<Option<T>, Error>
        where T: Cherry + Sync {
        assert!(args.count > 0);
        let sql = SqlBuilder::select_from(T::table())
            .and_where_eq(eq_field, "?")
            .sql()?;

        let x = sqlx::query_with(sql.as_str(), args.inner)
            // .map(|row| T::from_row(&DBRows::MySqlRow(row)))
            .fetch_optional(mysql::pool(Self::key())?)
            .await?;

        match x {
            Some(row) => Ok(Some(T::from_row(&WrapRows::MySqlRow(row))?)),
            _ => Ok(None)
        }
    }

    async fn select_in<'a, T>(&self, in_field: &str, args: MySqlArguments<'a>)
                              -> Result<Vec<T>, Error>
        where T: Cherry + Sync {
        assert!(args.count > 0);
        let sql = SqlBuilder::select_from(T::table())
            .and_where_in(in_field, vec!["?"; args.count].as_slice())
            .sql()?;
        mysql::fetch(Self::key(), sql, args).await
    }

    async fn select_list<'a, T>(&self, sql: &str, args: MySqlArguments<'a>)
                                -> Result<Vec<T>, Error>
        where T: Cherry + Sync {
        mysql::fetch(Self::key(), sql, args).await
    }

    async fn update<'a, T>(&self, set_fields: &[&str], eq_fields: &[&str], args: MySqlArguments<'a>,
                           tx: Option<&mut Transaction<'a, MySql>>)
        -> Result<u64, Error>
        where T: Cherry + Sync {
        let mut sql = SqlBuilder::update_table(T::table());
        set_fields.iter().for_each(|f| { sql.set(f, "?"); });
        eq_fields.iter().for_each(|x| { sql.and_where_eq(x, "?"); });
        let sql = sql.sql()?;

        let x = mysql::execute(Self::key(), sql, args, tx).await?;
        Ok(x.rows_affected())
    }

    async fn delete<'a, T>(&self, eq_fields: &[&str], args: MySqlArguments<'a>,
                           tx: Option<&mut Transaction<'a, MySql>>)
                           -> Result<u64, Error>
        where T: Cherry + Sync {
        let mut sql = SqlBuilder::delete_from(T::table());
        eq_fields.iter().for_each(|f| { sql.and_where_eq(f, "?"); });
        let sql = sql.sql()?;
        let x = mysql::execute(Self::key(), sql, args, tx).await?;
        Ok(x.rows_affected())
    }

    async fn execute<'a, S>(&self, sql: S, args: MySqlArguments<'a>,
                           tx: Option<&mut Transaction<'a, MySql>>)
                           -> Result<u64, Error>
        where S: AsRef<str> + Sync + Send {
        let x = mysql::execute(Self::key(), sql, args, tx).await?;
        Ok(x.rows_affected())
    }

    async fn begin<'a>(&self) -> Result<Transaction<'a, MySql>, Error> {
        Ok(pool(Self::key())?.begin().await?)
    }

    fn key() -> &'static str {
        "default"
    }
}
