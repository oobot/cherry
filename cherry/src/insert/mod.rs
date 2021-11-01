use std::any::TypeId;

use anyhow::anyhow;
use sql_builder::SqlBuilder;

use crate::{Cherry, Result};
use crate::adapt::arguments::Arguments;
use crate::adapt::query_result::QueryResult;
use crate::adapt::transaction::Transaction;
use crate::connection;
use crate::execute::{self, Data, TxMode};

pub struct Insert<'a> {
    datasource: TypeId,
    sql_builder: SqlBuilder,
    columns: Vec<&'static str>,
    replace: Option<(String, String)>,
    arguments: Arguments<'a>,
    tx: TxMode<'a>,
    size: usize,
}

impl<'a> Insert<'a> {
    fn new<T>(datasource: TypeId) -> Self where T: Cherry {
        Self {
            datasource, sql_builder: SqlBuilder::insert_into(T::table()),
            replace: None, tx: TxMode::None,
            arguments: Arguments::new(), size: 0, columns: T::columns()
        }
    }

    pub(crate) fn insert<T>(ds: TypeId, v: &'a T) -> Self where T: Cherry {
        let mut t = Self::new::<T>(ds);
        t.size = 1;
        v.arguments(&mut t.arguments);
        t
    }

    pub(crate) fn insert_bulk<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(ds);
        t.size = v.len();
        v.iter().for_each(|v| v.arguments(&mut t.arguments) );
        t
    }

    pub(crate) fn insert_ignore<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(ds);
        t.size = v.len();
        t.replace = Some(("INSERT".into(), "INSERT IGNORE".into()));
        v.iter().for_each(|v| v.arguments(&mut t.arguments) );
        t
    }

    pub(crate) fn insert_replace<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(ds);
        t.size = v.len();
        t.replace = Some(("INSERT INTO".into(), "INSERT REPLACE INTO".into()));
        v.iter().for_each(|v| v.arguments(&mut t.arguments) );
        t
    }

    pub fn tx(mut self, tx: &'a mut Transaction<'a>) -> Self {
        self.tx_ref(tx);
        self
    }

    fn tx_ref(&mut self, tx: &'a mut Transaction<'a>) -> &Self {
        self.tx = TxMode::Manual(tx);
        self
    }

    pub fn tx_auto(mut self) -> Self {
        self.tx_auto_ref();
        self
    }

    fn tx_auto_ref(&mut self) -> &Self {
        self.tx = TxMode::Auto;
        self
    }

    fn build_sql(&mut self) -> Result<String> {
        let holders = vec!["?"; self.columns.len()];
        self.sql_builder.fields(self.columns.as_slice());
        (0..self.size).for_each(|_| {
            self.sql_builder.values(holders.as_slice());
        });
        let mut sql = self.sql_builder.sql()?;
        if let Some((src, target)) = &self.replace {
            sql = sql.replacen(src.as_str(), target.as_str(), 1);
        }
        Ok(sql)
    }

    pub async fn execute(mut self) -> Result<QueryResult>  {
        let data = Data {
            datasource: self.datasource,
            sql: self.build_sql()?,
            arguments: self.arguments,
            tx: self.tx
        };
        execute::execute(data).await
    }

}

pub struct InsertUpdate<'a> {
    insert: Insert<'a>,
    fields: Vec<String>,
}

impl<'a> InsertUpdate<'a> {

    fn new<T>(ds: TypeId) -> Self where T: Cherry {
        Self { insert: Insert::new::<T>(ds), fields: vec![] }
    }

    pub(crate) fn insert_update<T>(ds: TypeId, v: &'a [T]) -> Self where T: Cherry {
        let mut t = Self::new::<T>(ds);
        t.insert.size = v.len();
        v.iter().for_each(|v| v.arguments(&mut t.insert.arguments) );
        t
    }

    pub fn field<T: AsRef<str>>(mut self, f: T) -> Self {
        self.field_ref(f);
        self
    }

    pub fn field_ref<T: AsRef<str>>(&mut self, f: T) -> &Self {
        self.fields.push(f.as_ref().to_owned());
        self
    }

    pub fn fields<T: AsRef<str>>(mut self, f: &[T]) -> Self {
        self.fields_ref(f);
        self
    }

    pub fn fields_ref<T: AsRef<str>>(&mut self, f: &[T]) -> &Self {
        f.iter().for_each(|f| {
            self.fields.push(f.as_ref().to_owned());
        });
        self
    }

    pub fn tx(mut self, tx: &'a mut Transaction<'a>) -> Self {
        self.insert.tx_ref(tx);
        self
    }

    pub fn tx_auto(mut self) -> Self {
        self.insert.tx_auto_ref();
        self
    }

    pub async fn execute(mut self) -> Result<QueryResult>  {
        if self.fields.is_empty() {
            return Err(anyhow!("Empty update fields."));
        }
        let insert = self.insert.build_sql()?.strip_suffix(";")
            .ok_or(anyhow!("Empty sql. This wasn’t supposed to happen."))?
            .to_owned();
        let update = self.fields.iter().map(|x| format!("{0} = new.{0}, ", x))
            .collect::<String>()
            .strip_suffix(",")
            .ok_or(anyhow!("Empty sql. This wasn’t supposed to happen."))?
            .to_owned();

        let data = Data {
            datasource: self.insert.datasource,
            sql: format!("{} AS new ON DUPLICATE KEY UPDATE {};", insert, update),
            arguments: self.insert.arguments,
            tx: self.insert.tx
        };

        execute::execute(data).await
    }

}
