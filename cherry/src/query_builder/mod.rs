pub mod select;
pub mod insert;
pub mod where_clause;
pub mod set_clause;
pub mod end;

#[derive(Copy, Clone)]
pub enum TargetQuery {
    MySql,
    Postgres,
    Sqlite,
}

impl TargetQuery {

    pub(crate) fn quote(&self, s: &str) -> String {
        match self {
            TargetQuery::MySql => format!("`{}`", s),
            TargetQuery::Postgres | TargetQuery::Sqlite => format!(r#""{}""#, s),
        }
    }
}