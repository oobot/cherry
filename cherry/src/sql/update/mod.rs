use crate::sql::set_clause::SetClause;
use crate::sql::TargetQuery;
use crate::sql::where_clause::WhereClause;

pub struct UpdateBuilder<'a> {
    target: TargetQuery,
    table: &'a str,
    pub(crate) set_clause: SetClause<'a>,
    pub(crate) where_clause: WhereClause<'a>,
}

impl<'a> UpdateBuilder<'a> {

    pub fn from(target: TargetQuery, table: &'a str) -> Self {
        Self {
            target, table,
            set_clause: SetClause::from(target),
            where_clause: WhereClause::from(target),
        }
    }

    pub fn as_sql(&self) -> String {
        let table = self.target.quote(self.table);
        let set_clause = self.set_clause.as_clause().unwrap_or_default();
        match self.where_clause.as_sql() {
            Some(v) => format!("UPDATE {} SET {} WHERE {}", table, set_clause, v),
            _ => format!("UPDATE {} SET {}", table, set_clause),
        }
    }

}
