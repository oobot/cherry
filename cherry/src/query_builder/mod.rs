pub mod select;
pub mod insert;
pub mod r#where;
pub mod end;

#[derive(Copy, Clone)]
pub enum TargetDatabase {
    Postgres,
    MySql,
    Sqlite,
}

impl TargetDatabase {

    pub(crate) fn wrap(&self, word: &str) -> String {
        match self {
            TargetDatabase::MySql => format!("`{}`", word),
            TargetDatabase::Postgres | TargetDatabase::Sqlite => format!(r#""{}""#, word),
        }
    }
}