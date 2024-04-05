use crate::dialect::Dialect;

pub struct Select {
    table: String,
    dialect: Dialect,
}

impl Select {
    pub fn from(table: String, dialect: Dialect) -> Self {
        todo!()
    }

    pub fn to_sql(&self) -> String {
        todo!()
    }
}