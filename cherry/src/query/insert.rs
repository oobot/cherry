use log::debug;
use sql_builder::SqlBuilder;

use crate::{Cherry, connection, gen_execute};
use crate::query::query_builder::QueryBuilder;
use crate::types::{QueryResult, Result, Transaction};

pub struct Insert<'a> {
    pub(crate) query: QueryBuilder<'a>,
    pub(crate) columns: Vec<&'static str>,
    pub(crate) replace: Option<(String, String)>,
    pub(crate) size: usize,
}

impl<'a> Insert<'a> {

    pub(crate) fn new<T>(ds: &'a str) -> Self where T: Cherry {
        Self {
            query: QueryBuilder::new::<T>(ds, SqlBuilder::insert_into(T::table())),
            columns: T::columns(),
            replace: None,
            size: 0,
        }
    }

    pub(crate) fn insert<T>(ds: &'a str, v: &'a T) -> Self where T: Cherry {
        let mut t = Self::new::<T>(ds);
        t.size = 1;
        v.arguments(&mut t.query.arguments);
        t
    }

    pub(crate) fn insert_bulk<T>(ds: &'a str, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(ds);
        t.size = v.len();
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    pub(crate) fn insert_ignore<T>(ds: &'a str, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(ds);
        t.size = v.len();
        t.replace = Some(("INSERT".into(), "INSERT IGNORE".into()));
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    pub(crate) fn insert_replace<T>(ds: &'a str, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(ds);
        t.size = v.len();
        t.replace = Some(("INSERT INTO".into(), "REPLACE INTO".into()));
        v.iter().for_each(|v| v.arguments(&mut t.query.arguments) );
        t
    }

    fn build_sql(&mut self) -> Result<String> {
        let holders = vec!["?"; self.columns.len()];
        self.query.sql_builder.fields(self.columns.as_slice());
        (0..self.size).for_each(|_| {
            self.query.sql_builder.values(holders.as_slice());
        });
        let mut sql = self.query.sql_builder.sql()?;
        if let Some((src, target)) = &self.replace {
            sql = sql.replacen(src.as_str(), target.as_str(), 1);
        }
        Ok(sql)
    }

    gen_execute!();

}
