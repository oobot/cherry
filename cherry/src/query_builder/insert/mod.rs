use crate::query_builder::insert::Conflict::Ignore;
use crate::query_builder::TargetDatabase;
use crate::query_builder::r#where::WhereStatement;

pub struct InsertBuilder<'a> {
    db: TargetDatabase,
    table: &'a str,
    columns: Vec<&'a str>,
    rows: usize,
    conflict: Conflict,
    conflict_columns: Vec<&'a str>,
    update_columns: Vec<&'a str>,
    pub(crate) r#where: WhereStatement<'a>,
}

impl<'a> InsertBuilder<'a> {

    pub fn from(db: TargetDatabase, table: &'a str, columns: Vec<&'a str>, rows: usize) -> Self {
        Self {
            db, table, columns, rows,
            conflict: Conflict::None,
            conflict_columns: vec![],
            update_columns: vec![],
            r#where: WhereStatement::from(db),
        }
    }

    pub fn conflict(&mut self, conflict: Conflict) {
        self.conflict = conflict;
    }

    pub fn add_conflict_columns(&mut self, columns: &'a [&'a str]) {
        self.conflict_columns.extend(columns);
    }

    pub fn add_update_columns(&mut self, columns: &'a [&'a str]) {
        self.update_columns.extend(columns);
    }

    // pub fn conflict_update() {}

    // pub fn conflict_where() {}



    pub fn as_sql(&self) -> String {
        todo!()
    }
}

#[derive(Copy, Clone)]
pub enum Conflict {
    None,
    Ignore,
    Update,
    Replace
}
