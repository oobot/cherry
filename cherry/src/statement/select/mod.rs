use crate::statement::end::EndStatement;
use crate::statement::r#where::WhereStatement;

pub struct SelectStatement<'a> {
    pub table: &'a str,
    pub columns: Vec<&'a str>,
    pub r#where: WhereStatement<'a>,
    pub end: EndStatement<'a>,
}

impl<'a> SelectStatement<'a> {

    pub fn from(table: &'a str) -> Self {
        Self { table, columns: vec![], r#where: WhereStatement::new(), end: EndStatement::new(), }
    }

    pub fn sql(&self) -> String {
        let columns = match self.columns.is_empty() {
            true => "*".to_string(),
            _ => self.columns.join(", "),
        };

        let mut vec: Vec<String> = vec!["SELECT".into(), columns, "FROM".into(), self.table.into()];

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
