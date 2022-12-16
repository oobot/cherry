use crate::query_builder::insert::Conflict::*;
use crate::query_builder::set_clause::SetClause;
use crate::query_builder::TargetQuery;
use crate::query_builder::where_clause::WhereClause;

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
    conflict: (Conflict, Vec<&'a str>),
    pub(crate) set_clause: SetClause<'a>,
    pub(crate) where_clause: WhereClause<'a>,
}

impl<'a> InsertBuilder<'a> {

    pub fn from(target: TargetQuery, table: &'a str, columns: Vec<&'a str>, rows: usize) -> Self {
        Self {
            target, table, columns, rows,
            conflict: (None, vec![]),
            set_clause: SetClause::from(target),
            where_clause: WhereClause::from(target),
        }
    }

    pub fn conflict(&mut self, conflict: Conflict) {
        self.conflict.0 = conflict;
    }

    pub fn add_conflict_columns(&mut self, columns: &'a [&'a str]) {
        self.conflict.1.extend(columns);
    }

    pub fn as_sql(&self) -> String {
        match self.target {
            TargetQuery::MySql => self.mysql(),
            TargetQuery::Sqlite => self.sqlite(),
            TargetQuery::Postgres => self.postgres(),
        }
    }

    fn mysql(&self) -> String {
        match self.conflict.0 {
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
            // https://dev.mysql.com/doc/refman/8.0/en/insert-on-duplicate.html
            Update => format!(
                "INSERT INTO {} ({}) VALUES ({}) AS new ON DUPLICATE KEY UPDATE {}",
                self.table(), self.columns(), self.values_holder(),
                self.set_clause.as_clause().unwrap_or_default(),
            ),
        }
    }

    fn sqlite(&self) -> String {
        match self.conflict.0 {
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
                "INSERT INTO {} ({}) VALUES {} ON CONFLICT{} DO UPDATE SET {}{}",
                self.table(), self.columns(), self.values_holder(),
                self.conflict_columns(),
                self.set_clause.as_clause().unwrap_or_default(),
                self.where_clause.as_sql().map(|v| format!(" WHERE {}", v)).unwrap_or_default()
            ),
        }
    }

    fn postgres(&self) -> String {
        match self.conflict.0 {
            None => format!(
                "INSERT INTO {} ({}) VALUES {}",
                self.table(), self.columns(), self.values_holder(),
            ),
            Ignore => format!(
                "INSERT INTO {} ({}) VALUES {} ON CONFLICT{} DO NOTHING",
                self.table(), self.columns(), self.values_holder(),
                self.conflict_columns(),
            ),
            Update => format!(
                "INSERT INTO {} ({}) VALUES {} ON CONFLICT{} DO UPDATE SET {}{}",
                self.table(), self.columns(), self.values_holder(),
                self.conflict_columns(),
                self.set_clause.as_clause().unwrap_or_default(),
                self.where_clause.as_sql().map(|v| format!(" WHERE {}", v)).unwrap_or_default()
            ),
            Replace => unreachable!(),
        }
    }

    fn table(&self) -> String {
        self.target.quote(self.table)
    }

    fn columns(&self) -> String {
        self.columns.iter()
            .map(|v| self.target.quote(v))
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
        let columns = self.conflict.1.iter()
            .map(|c| self.target.quote(c))
            .collect::<Vec<String>>()
            .join(", ");
        format!("({})", columns)
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
        let left = r#"INSERT INTO "user" ("id", "name") VALUES (?, ?)"#;
        assert_eq!(left, builder.as_sql());

        builder.rows = 2;
        builder.conflict(Update);
        builder.add_conflict_columns(&["id"]);
        builder.add_update_columns(&["id", "name"]);
        let sql = builder.as_sql();
        let left = r#"INSERT INTO "user" ("id", "name") VALUES (?, ?), (?, ?) ON CONFLICT("id") DO UPDATE SET "id" = excluded."id", "name" = excluded."name""#;
        assert_eq!(left, builder.as_sql());
    }
}