pub mod select;
pub mod insert;
pub mod r#where;
pub mod end;

#[derive(Copy, Clone)]
pub enum TargetQuery {
    MySql,
    Postgres,
    Sqlite,
}

impl TargetQuery {

    pub(crate) fn wrap(&self, word: &str) -> String {
        match self {
            TargetQuery::MySql => format!("`{}`", word),
            TargetQuery::Postgres | TargetQuery::Sqlite => format!(r#""{}""#, word),
        }
    }
}