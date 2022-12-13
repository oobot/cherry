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

        todo!()
    }


}

