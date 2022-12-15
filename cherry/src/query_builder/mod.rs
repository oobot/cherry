pub mod select;
pub mod insert;
pub mod r#where;
pub mod end;

#[derive(Copy, Clone)]
pub enum QueryDatabase {
    Postgres,
    MySql,
    Sqlite,
}

impl QueryDatabase {

    pub(crate) fn wrap_key(&self, word: &str) -> String {
        match self {
            QueryDatabase::MySql => format!("`{}`", word),
            QueryDatabase::Postgres | QueryDatabase::Sqlite => format!(r#""{}""#, word),
        }
    }
}