use crate::query_builder::end::EndClause;
use crate::query_builder::TargetQuery;
use crate::query_builder::where_clause::WhereClause;

pub struct SelectBuilder<'a> {
    target: TargetQuery,
    table: &'a str,
    columns: Vec<&'a str>,
    pub where_clause: WhereClause<'a>,
    pub end: EndClause<'a>,
}

impl<'a> SelectBuilder<'a> {

    pub fn from(db: TargetQuery, table: &'a str) -> Self {
        Self {
            target: db, table,
            columns: vec![],
            where_clause: WhereClause::from(db),
            end: EndClause::from(db),
        }
    }

    pub fn sql(&self) -> String {
        let columns = match self.columns.is_empty() {
            true => "*".to_string(),
            _ => self.columns.iter().map(|v| self.target.quote(v))
                .collect::<Vec<String>>().join(", "),
        };

        let mut vec: Vec<String> = vec![
            format!("SELECT {} FROM {}", columns, self.target.quote(self.table))
        ];

        let where_clause = self.where_clause.as_sql();
        if let Some(statement) = where_clause {
            vec.push("WHERE".into());
            vec.push(statement);
        }

        let end = self.end.as_clause();
        if let Some(statement) = end {
            vec.push(statement);
        }
        vec.join(" ")
    }

}
