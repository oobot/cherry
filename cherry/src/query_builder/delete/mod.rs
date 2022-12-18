use crate::query_builder::TargetQuery;
use crate::query_builder::where_clause::WhereClause;

pub struct DeleteBuilder<'a> {
    target: TargetQuery,
    table: &'a str,
    pub(crate) where_clause: WhereClause<'a>,
}

impl<'a> DeleteBuilder<'a> {

    pub fn from(target: TargetQuery, table: &'a str) -> Self {
        Self {
            target, table,
            where_clause: WhereClause::from(target),
        }
    }

    pub fn as_sql(&self) -> String {
        let table = self.target.quote(self.table);
        match self.where_clause.as_sql() {
            Some(v) => format!("DELETE FROM {} WHERE {}", table, v),
            _ => format!("DELETE FROM {}", table),
        }
    }

}
