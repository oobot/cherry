use crate::sql::TargetDatabase;
use crate::sql::where_condition::Condition::*;

pub enum Condition<'a> {
    And(Vec<Condition<'a>>),
    Or(Vec<Condition<'a>>),

    AndEq(&'a str),
    OrEq(&'a str),

    AndGe(&'a str),
    OrGe(&'a str),
    AndGt(&'a str),
    OrGt(&'a str),

    AndLe(&'a str),
    OrLe(&'a str),
    AndLt(&'a str),
    OrLt(&'a str),

    AndIsNull(&'a str),
    OrIsNull(&'a str),
    AndIsNotNull(&'a str),
    OrIsNotNull(&'a str),

    AndBetween(&'a str),
    OrBetween(&'a str),
    AndNotBetween(&'a str),
    OrNotBetween(&'a str),

    AndIn(&'a str, usize),
    OrIn(&'a str, usize),
    AndNotIn(&'a str, usize),
    OrNotIn(&'a str, usize),

    AndEqColumn(&'a str),
    OrEqColumn(&'a str),

    AndGeColumn(&'a str),
    OrGeColumn(&'a str),
    AndGtColumn(&'a str),
    OrGtColumn(&'a str),

    AndLeColumn(&'a str),
    OrLeColumn(&'a str),
    AndLtColumn(&'a str),
    OrLtColumn(&'a str),

    AndColumnIsNull(&'a str),
    OrColumnIsNull(&'a str),
    AndColumnIsNotNull(&'a str),
    OrColumnIsNotNull(&'a str),
}

impl<'a> Condition<'a> {

    pub(crate) fn as_sql(&self, db: TargetDatabase) -> String {
        match &self {
            And(_) | Or(_) => unreachable!(),
            AndEq(c) => format!("{} = ?", db.quote(c)),
            OrEq(c) => format!("{} = ?", db.quote(c)),
            AndGe(c) => format!("{} >= ?", db.quote(c)),
            OrGe(c) => format!("{} >= ?", db.quote(c)),
            AndGt(c) => format!("{} > ?", db.quote(c)),
            OrGt(c) => format!("{} > ?", db.quote(c)),
            AndLe(c) => format!("{} =< ?", db.quote(c)),
            OrLe(c) => format!("{} =< ?", db.quote(c)),
            AndLt(c) => format!("{} < ?", db.quote(c)),
            OrLt(c) => format!("{} < ?", db.quote(c)),
            AndIsNull(c) => format!("{} IS NULL", db.quote(c)),
            OrIsNull(c) => format!("{} IS NULL", db.quote(c)),
            AndIsNotNull(c) => format!("{} IS NOT NULL", db.quote(c)),
            OrIsNotNull(c) => format!("{} IS NOT NULL", db.quote(c)),
            AndBetween(c) => format!("{} BETWEEN ? AND ?", db.quote(c)),
            OrBetween(c) => format!("{} BETWEEN ? AND ?", db.quote(c)),
            AndNotBetween(c) => format!("{} NOT BETWEEN ? AND ?", db.quote(c)),
            OrNotBetween(c) => format!("{} NOT BETWEEN ? AND ?", db.quote(c)),
            AndIn(c, n) => format!("{} IN ({})", db.quote(c), vec!["?"; *n].join(", ")),
            AndNotIn(c, n) => format!("{} NOT IN ({})", db.quote(c), vec!["?"; *n].join(", ")),
            OrIn(c, n) => format!("{} IN ({})", db.quote(c), vec!["?"; *n].join(", ")),
            OrNotIn(c, n) => format!("{} NOT IN ({})", db.quote(c), vec!["?"; *n].join(", ")),

            AndEqColumn(c) => format!("{} = {}", self.target_column(db, c), c),
            OrEqColumn(c) => format!("{} = {}", self.target_column(db, c), c),
            AndGeColumn(c) => format!("{} >= {}", self.target_column(db, c), c),
            OrGeColumn(c) => format!("{} >= {}", self.target_column(db, c), c),
            AndGtColumn(c) => format!("{} > {}", self.target_column(db, c), c),
            OrGtColumn(c) => format!("{} > {}", self.target_column(db, c), c),
            AndLeColumn(c) => format!("{} <= {}", self.target_column(db, c), c),
            OrLeColumn(c) => format!("{} <= {}", self.target_column(db, c), c),
            AndLtColumn(c) => format!("{} < {}", self.target_column(db, c), c),
            OrLtColumn(c) => format!("{} < {}", self.target_column(db, c), c),
            AndColumnIsNull(c) => format!("{} IS NULL", self.target_column(db, c)),
            OrColumnIsNull(c) => format!("{} IS NULL", self.target_column(db, c)),
            AndColumnIsNotNull(c) => format!("{} IS NOT NULL", self.target_column(db, c)),
            OrColumnIsNotNull(c) => format!("{} IS NOT NULL", self.target_column(db, c)),
        }
    }

    fn target_column(&self, db: TargetDatabase, c: &str) -> String {
        match db {
            // https://dev.mysql.com/doc/refman/8.0/en/insert-on-duplicate.html
            TargetDatabase::MySql => format!(r#"new.{}"#, db.quote(c)),
            TargetDatabase::Postgres | TargetDatabase::Sqlite => format!(r#"excluded.{0}"#, db.quote(c)),
        }
    }

    pub(crate) fn and_or(&self) -> bool {
        match &self {
            And(_) | AndEq(_) |
            AndGe(_) | AndGt(_) | AndLe(_) | AndLt(_) |
            AndIn(_, _) | AndNotIn(_, _) |
            AndIsNull(_) | AndIsNotNull(_) |
            AndBetween(_) | AndNotBetween(_) |
            AndEqColumn(_) |
            AndGeColumn(_) | AndGtColumn(_) | AndLeColumn(_) | AndLtColumn(_) |
            AndColumnIsNull(_) | AndColumnIsNotNull(_) => true,

            Or(_) | OrEq(_) |
            OrGe(_) | OrGt(_) | OrLe(_) | OrLt(_) |
            OrIn(_, _) | OrNotIn(_, _) |
            OrIsNull(_) | OrIsNotNull(_) |
            OrBetween(_) | OrNotBetween(_) |
            OrEqColumn(_) |
            OrGeColumn(_) | OrGtColumn(_) | OrLeColumn(_) | OrLtColumn(_) |
            OrColumnIsNull(_) | OrColumnIsNotNull(_) => false,
        }
    }
}
