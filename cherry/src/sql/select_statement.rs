use crate::sql::filter_statement::FilterStatement;

pub struct SelectStatement<'a> {
    pub table: &'a str,
    pub columns: Vec<&'a str>,
    pub filter: FilterStatement<'a>,
}

impl<'a> SelectStatement<'a> {

    pub fn from(table: &'a str) -> Self {
        Self { table, columns: vec![], filter: FilterStatement::new(), }
    }

    pub fn sql(&self) -> String {
        let columns = match self.columns.is_empty() {
            true => "*".to_string(),
            _ => self.columns.join(", "),
        };
        match self.filter.as_statement() {
            Some(v) => format!("SELECT {} FROM {} WHERE {}", columns, self.table, v),
            None => format!("SELECT {} FROM {}", columns, self.table),
        }
    }

}

