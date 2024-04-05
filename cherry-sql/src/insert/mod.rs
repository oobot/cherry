use crate::dialect::Dialect;

pub struct Insert {
    dialect: Dialect,
    table: String,
    columns: Vec<String>,
    value_size: usize,
    // on conflict
    // where
}

impl Insert {
    pub fn from(dialect: Dialect, table: String, columns: Vec<String>) -> Self {
        todo!()
    }

    pub fn to_sql(&self) -> String {
        todo!()
    }

}