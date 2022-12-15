use crate::query_builder::end::EndStatement;
use crate::query_builder::TargetDatabase;
use crate::query_builder::r#where::WhereStatement;

pub struct SelectBuilder<'a> {
    db: TargetDatabase,
    pub table: &'a str,
    pub columns: Vec<&'a str>,
    pub r#where: WhereStatement<'a>,
    pub end: EndStatement<'a>,
}

impl<'a> SelectBuilder<'a> {

    pub fn from(db: TargetDatabase, table: &'a str) -> Self {
        Self { db, table,
            columns: vec![],
            r#where: WhereStatement::from(db),
            end: EndStatement::from(db),
        }
    }

    pub fn sql(&self) -> String {
        let columns = match self.columns.is_empty() {
            true => "*".to_string(),
            _ => self.columns.iter().map(|v| self.db.wrap(v)).join(", "),
        };

        let mut vec: Vec<String> = vec![
            format!("SELECT {} FROM {}", columns, self.db.wrap(self.table))
        ];

        let where_ = self.r#where.as_statement();
        if let Some(statement) = where_ {
            vec.push("WHERE".into());
            vec.push(statement);
        }

        let end = self.end.as_statement();
        if let Some(statement) = end {
            vec.push(statement);
        }
        vec.join(" ")
    }

}
