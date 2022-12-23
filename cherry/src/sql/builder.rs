use crate::sql::{InsertOnConflict::{self, *}, SqlType, TargetDatabase};
use crate::sql::end_clause::{EndClause, EndSection};
use crate::sql::select_column::{Column, SelectColumnClause};
use crate::sql::update_set_clause::{UpdateSetClause, UpdateSetSection};
use crate::sql::where_clause::WhereClause;
use crate::sql::where_condition::Condition;

pub struct SqlBuilder<'a> {
    pub(crate) db: TargetDatabase,
    pub(crate) sql_type: SqlType,
    pub(crate) table: &'a str,
    pub(crate) table_columns: Vec<&'a str>,

    pub(crate) rows: usize, // insert row count
    pub(crate) conflict: (InsertOnConflict, Vec<&'a str>), // conflict action and conflict columns

    pub(crate) update_set_clause: UpdateSetClause<'a>,
    pub(crate) select_column_clause: SelectColumnClause<'a>,
    pub(crate) where_clause: WhereClause<'a>,
    pub(crate) end_clause: EndClause<'a>,
}

impl<'a> SqlBuilder<'a> {

    pub(crate) fn from_insert(db: TargetDatabase, table: &'a str, columns: Vec<&'a str>, rows: usize) -> Self {
        Self {
            db,
            sql_type: SqlType::Insert,
            table,
            table_columns: columns,
            rows,
            conflict: (None, vec![]),
            update_set_clause: Default::default(),
            select_column_clause: Default::default(),
            where_clause: Default::default(),
            end_clause: Default::default(),
        }
    }

    pub(crate) fn from_select(db: TargetDatabase, table: &'a str, columns: Vec<&'a str>) -> Self {
        Self {
            db,
            sql_type: SqlType::Select,
            table,
            table_columns: columns,
            rows: 0,
            conflict: (None, vec![]),
            update_set_clause: Default::default(),
            select_column_clause: Default::default(),
            where_clause: Default::default(),
            end_clause: Default::default(),
        }
    }

    pub(crate) fn from_update(db: TargetDatabase, table: &'a str) -> Self {
        Self {
            db,
            sql_type: SqlType::Update,
            table,
            table_columns: vec![],
            rows: 0,
            conflict: (None, vec![]),
            update_set_clause: Default::default(),
            select_column_clause: Default::default(),
            where_clause: Default::default(),
            end_clause: Default::default(),
        }
    }

    pub(crate) fn from_delete(db: TargetDatabase, table: &'a str) -> Self {
        Self {
            db,
            sql_type: SqlType::Delete,
            table,
            table_columns: vec![],
            rows: 0,
            conflict: (None, vec![]),
            update_set_clause: Default::default(),
            select_column_clause: Default::default(),
            where_clause: Default::default(),
            end_clause: Default::default(),
        }
    }

    pub(crate) fn conflict_with(&mut self, conflict: InsertOnConflict) {
        self.conflict.0 = conflict;
    }

    #[allow(dead_code)]
    pub(crate) fn add_conflict_column(&mut self, column: &'a str) {
        self.conflict.1.push(column);
    }

    pub(crate) fn add_update_section(&mut self, section: UpdateSetSection<'a>) {
        self.update_set_clause.add(section);
    }

    pub(crate) fn add_select_column(&mut self, column: Column<'a>) {
        self.select_column_clause.add(column);
    }

    pub(crate) fn add_where(&mut self, condition: Condition<'a>) {
        self.where_clause.add(condition);
    }

    pub(crate) fn add_end_section(&mut self, section: EndSection<'a>) {
        self.end_clause.add(section);
    }

    pub(crate) fn surround_where(&mut self) {
        self.where_clause.make_temp();
    }

    pub(crate) fn take_surround(&mut self) -> Vec<Condition<'a>> {
        self.where_clause.take_temp()
    }


    pub(crate) fn table(&self) -> String {
        self.db.quote(self.table)
    }

    pub(crate) fn as_sql(&self) -> String {
        match self.sql_type {
            SqlType::Insert => self.as_insert_sql(),
            SqlType::Update => self.as_update_sql(),
            SqlType::Delete => self.as_delete_sql(),
            SqlType::Select => self.as_select_sql(),
        }
    }

    fn as_select_sql(&self) -> String {
        let mut vec: Vec<String> = vec![
            format!("SELECT {} FROM {}", self.select_column_clause.as_sql(self.db), self.table())
        ];

        if let Some(statement) = self.where_clause.as_sql(self.db) {
            vec.push("WHERE".into());
            vec.push(statement);
        }

        if let Some(statement) = self.end_clause.as_clause(self.db) {
            vec.push(statement);
        }
        vec.join(" ")
    }

    fn as_update_sql(&self) -> String {
        let set_clause = self.update_set_clause.as_clause(self.db).unwrap_or_default();
        match self.where_clause.as_sql(self.db) {
            Some(v) => format!("UPDATE {} SET {} WHERE {}", self.table(), set_clause, v),
            _ => format!("UPDATE {} SET {}", self.table(), set_clause),
        }
    }

    fn as_delete_sql(&self) -> String {
        match self.where_clause.as_sql(self.db) {
            Some(v) => format!("DELETE FROM {} WHERE {}", self.table(), v),
            _ => format!("DELETE FROM {}", self.table()),
        }
    }

    // *********************************** Insert *************************************

    pub(crate) fn as_insert_sql(&self) -> String {
        match self.db {
            TargetDatabase::MySql => self.mysql(),
            TargetDatabase::Sqlite => self.sqlite(),
            TargetDatabase::Postgres => self.postgres(),
        }
    }

    fn mysql(&self) -> String {
        match self.conflict.0 {
            None => format!(
                "INSERT INTO {} ({}) VALUES {}",
                self.table(), self.table_columns(), self.values_holder(),
            ),
            Ignore => format!(
                "INSERT IGNORE INTO {} ({}) VALUES {}",
                self.table(), self.table_columns(), self.values_holder(),
            ),
            Replace => format!(
                "REPLACE INTO {} ({}) VALUES {}",
                self.table(), self.table_columns(), self.values_holder(),
            ),
            // https://dev.mysql.com/doc/refman/8.0/en/insert-on-duplicate.html
            Update => format!(
                "INSERT INTO {} ({}) VALUES ({}) AS new ON DUPLICATE KEY UPDATE {}",
                self.table(), self.table_columns(), self.values_holder(),
                self.update_set_clause.as_clause(self.db).unwrap_or_default(),
            ),
        }
    }

    fn sqlite(&self) -> String {
        match self.conflict.0 {
            None => format!(
                "INSERT INTO {} ({}) VALUES {}",
                self.table(), self.table_columns(), self.values_holder(),
            ),
            Ignore => format!(
                "INSERT OR IGNORE INTO {} ({}) VALUES {}",
                self.table(), self.table_columns(), self.values_holder(),
            ),
            Replace => format!(
                "INSERT OR REPLACE INTO {} ({}) VALUES {}",
                self.table(), self.table_columns(), self.values_holder(),
            ),
            Update => format!(
                "INSERT INTO {} ({}) VALUES {} ON CONFLICT{} DO UPDATE SET {}{}",
                self.table(), self.table_columns(), self.values_holder(),
                self.conflict_columns(),
                self.update_set_clause.as_clause(self.db).unwrap_or_default(),
                self.where_clause.as_sql(self.db).map(|v| format!(" WHERE {}", v)).unwrap_or_default()
            ),
        }
    }

    fn postgres(&self) -> String {
        match self.conflict.0 {
            None => format!(
                "INSERT INTO {} ({}) VALUES {}",
                self.table(), self.table_columns(), self.values_holder(),
            ),
            Ignore => format!(
                "INSERT INTO {} ({}) VALUES {} ON CONFLICT{} DO NOTHING",
                self.table(), self.table_columns(), self.values_holder(),
                self.conflict_columns(),
            ),
            Update => format!(
                "INSERT INTO {} ({}) VALUES {} ON CONFLICT{} DO UPDATE SET {}{}",
                self.table(), self.table_columns(), self.values_holder(),
                self.conflict_columns(),
                self.update_set_clause.as_clause(self.db).unwrap_or_default(),
                self.where_clause.as_sql(self.db).map(|v| format!(" WHERE {}", v)).unwrap_or_default()
            ),
            Replace => unreachable!(),
        }
    }

    fn table_columns(&self) -> String {
        self.table_columns.iter()
            .map(|v| self.db.quote(v))
            .collect::<Vec<String>>().join(", ")
    }

    fn values_holder(&self) -> String {
        (0..self.rows)
            .map(|_| {
                let row = (0..self.table_columns.len())
                    .map(|_| "?")
                    .collect::<Vec<&str>>().join(", ");
                format!("({})", row)
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn conflict_columns(&self) -> String {
        let columns = self.conflict.1.iter()
            .map(|c| self.db.quote(c))
            .collect::<Vec<String>>()
            .join(", ");
        format!("({})", columns)
    }



}