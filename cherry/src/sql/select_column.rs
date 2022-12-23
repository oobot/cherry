use crate::sql::TargetDatabase;

#[derive(Default)]
pub struct SelectColumnClause<'a> {
    columns: Vec<Column<'a>>
}

impl<'a> SelectColumnClause<'a> {

    pub fn add(&mut self, column: Column<'a>) {
        self.columns.push(column);
    }

    pub fn as_sql(&self, db: TargetDatabase) -> String {
        match self.columns.is_empty() {
            true => "*".to_string(),
            _ => self.columns.iter()
                .map(|c|
                    match c {
                        Column::Column(c) => db.quote(c),
                        Column::Raw(r) => r.to_string(),
                    }
                )
                .collect::<Vec<String>>().join(", "),
        }
    }
}

pub enum Column<'a> {
    Column(&'a str),
    Raw(&'a str),
}