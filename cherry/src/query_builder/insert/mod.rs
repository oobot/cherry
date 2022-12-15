use crate::query_builder::insert::Conflict::*;
use crate::query_builder::r#where::WhereStatement;
use crate::query_builder::TargetQuery;

#[derive(Copy, Clone)]
pub enum Conflict {
    None,
    Ignore,
    Update,
    Replace
}

pub struct InsertBuilder<'a> {
    target: TargetQuery,
    table: &'a str,
    columns: Vec<&'a str>,
    rows: usize,
    conflict: Conflict,
    conflict_columns: Vec<&'a str>,
    update_columns: Vec<&'a str>,
    pub(crate) r#where: WhereStatement<'a>,
}

impl<'a> InsertBuilder<'a> {

    pub fn from(target: TargetQuery, table: &'a str, columns: Vec<&'a str>, rows: usize) -> Self {
        Self {
            target, table, columns, rows,
            conflict: None,
            conflict_columns: vec![],
            update_columns: vec![],
            r#where: WhereStatement::from(target),
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

    pub fn as_sql(&self) -> String {
        match self.target {
            TargetQuery::MySql => self.mysql(),
            TargetQuery::Sqlite => self.sqlite(),
            TargetQuery::Postgres => self.postgres(),
        }
    }

    fn mysql(&self) -> String {
        match self.conflict {
            None => format!(
                "INSERT INTO {} ({}) VALUES {}",
                self.table(), self.columns(), self.values_holder(),
            ),
            Ignore => format!(
                "INSERT IGNORE INTO {} ({}) VALUES {}",
                self.table(), self.columns(), self.values_holder(),
            ),
            Replace => format!(
                "REPLACE INTO {} ({}) VALUES {}",
                self.table(), self.columns(), self.values_holder(),
            ),
            Update => format!(
                "INSERT INTO {} ({}) VALUES ({}) ON DUPLICATE KEY UPDATE {}",
                self.table(), self.columns(), self.values_holder(), self.update_columns(),
            ),
        }
    }

    fn sqlite(&self) -> String {
        match self.conflict {
            None => format!(
                "INSERT INTO {} ({}) VALUES {}",
                self.table(), self.columns(), self.values_holder(),
            ),
            Ignore => format!(
                "INSERT OR IGNORE INTO {} ({}) VALUES {}",
                self.table(), self.columns(), self.values_holder(),
            ),
            Replace => format!(
                "INSERT OR REPLACE INTO {} ({}) VALUES {}",
                self.table(), self.columns(), self.values_holder(),
            ),
            Update => format!(
                "INSERT INTO {} ({}) VALUES {} ON CONFLICT({}) DO UPDATE SET {}",
                self.table(), self.columns(), self.values_holder(),
                self.conflict_columns(),
                self.update_columns()
            ),
        }
    }

    fn postgres(&self) -> String {
        match self.conflict {
            None => format!(
                "INSERT INTO {} ({}) VALUES {}",
                self.table(), self.columns(), self.values_holder(),
            ),
            Ignore => format!(
                "INSERT INTO {} ({}) VALUES {} ON CONFLICT({}) DO NOTHING",
                self.table(), self.columns(), self.values_holder(),
                self.conflict_columns(),
            ),
            Update => format!(
                "INSERT INTO {} ({}) VALUES {} ON CONFLICT({}) DO UPDATE SET {}",
                self.table(), self.columns(), self.values_holder(),
                self.conflict_columns(),
                self.update_columns()
            ),
            Replace => unreachable!(),
        }
    }

    fn table(&self) -> String {
        self.target.wrap(self.table)
    }

    fn columns(&self) -> String {
        self.columns.iter()
            .map(|v| self.target.wrap(v))
            .collect::<Vec<String>>().join(", ")
    }

    fn values_holder(&self) -> String {
        (0..self.rows)
            .map(|_| {
                let row = (0..self.columns.len())
                    .map(|_| "?")
                    .collect::<Vec<&str>>().join(", ");
                format!("({})", row)
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn conflict_columns(&self) -> String {
        self.conflict_columns.iter()
            .map(|c| self.target.wrap(c))
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn update_columns(&self) -> String {
        self.update_columns.iter()
            .map(|v|
                match self.target {
                    TargetQuery::MySql =>
                        format!(r#"{0} = VALUES({0})"#, self.target.wrap(v)),
                    TargetQuery::Postgres | TargetQuery::Sqlite =>
                        format!(r#"{0} = EXCLUDED.{0}""#, self.target.wrap(v)),
                }
            )
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite() {
        let mut builder = InsertBuilder::from(
            TargetQuery::Sqlite,
            "user",
            vec!["id", "name"],
            1
        );
        let sql = builder.as_sql();
        let left = r#"INSERT INTO "user" ("id", "name") VALUES (?, ?)"#;
        println!("{}", sql);
        assert_eq!(left, sql);
    }
}