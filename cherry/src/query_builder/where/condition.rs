use crate::query_builder::TargetDatabase;
use crate::query_builder::r#where::condition::Condition::*;

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

    AndIn(&'a str, usize),
    OrIn(&'a str, usize),
    AndNotIn(&'a str, usize),
    OrNotIn(&'a str, usize),

    AndIsNull(&'a str),
    OrIsNull(&'a str),

    AndBetween(&'a str),
    OrBetween(&'a str),
    AndNotBetween(&'a str),
    OrNotBetween(&'a str),
}

impl<'a> Condition<'a> {
    pub fn as_statement(&self, db: TargetDatabase) -> String {
        match &self {
            And(v) => format!("({})", Self::gen_all(v, db)),
            Or(v) => format!("({})", Self::gen_all(v, db)),
            AndEq(c) => format!("{} = ?", db.wrap(c)),
            OrEq(c) => format!("{} = ?", db.wrap(c)),
            AndGe(c) => format!("{} >= ?", db.wrap(c)),
            OrGe(c) => format!("{} >= ?", db.wrap(c)),
            AndGt(c) => format!("{} > ?", db.wrap(c)),
            OrGt(c) => format!("{} > ?", db.wrap(c)),
            AndLe(c) => format!("{} =< ?", db.wrap(c)),
            OrLe(c) => format!("{} =< ?", db.wrap(c)),
            AndLt(c) => format!("{} < ?", db.wrap(c)),
            OrLt(c) => format!("{} < ?", db.wrap(c)),
            AndIn(c, n) => format!("{} IN ({})", db.wrap(c), vec!["?"; *n].join(", ")),
            AndNotIn(c, n) => format!("{} NOT IN ({})", db.wrap(c), vec!["?"; *n].join(", ")),
            OrIn(c, n) => format!("{} IN ({})", db.wrap(c), vec!["?"; *n].join(", ")),
            OrNotIn(c, n) => format!("{} NOT IN ({})", db.wrap(c), vec!["?"; *n].join(", ")),
            AndIsNull(c) => format!("{} is null", db.wrap(c)),
            OrIsNull(c) => format!("{} is not null", db.wrap(c)),
            AndBetween(c) => format!("{} BETWEEN ? AND ?", db.wrap(c)),
            OrBetween(c) => format!("{} BETWEEN ? AND ?", db.wrap(c)),
            AndNotBetween(c) => format!("{} NOT BETWEEN ? AND ?", db.wrap(c)),
            OrNotBetween(c) => format!("{} NOT BETWEEN ? AND ?", db.wrap(c)),
        }
    }

    pub fn gen_all<'b>(vec: &'b [Condition<'b>], db: TargetDatabase) -> String {
        vec.iter().enumerate().map(|(i, c)| {
            match i {
                0 => c.as_statement(db),
                _ => match c.and_or() {
                    true => format!(" AND {}", c.as_statement(db)),
                    _ => format!(" OR {}", c.as_statement(db)),
                }
            }
        }).collect::<String>()
    }

    fn and_or(&self) -> bool {
        match &self {
            And(_) | AndEq(_) |
            AndGe(_) | AndGt(_) | AndLe(_) | AndLt(_) |
            AndIn(_, _) | AndNotIn(_, _) | AndIsNull(_) |
            AndBetween(_) | AndNotBetween(_) => true,

            Or(_) | OrEq(_) |
            OrGe(_) | OrGt(_) | OrLe(_) | OrLt(_) |
            OrIn(_, _) | OrNotIn(_, _) | OrIsNull(_) |
            OrBetween(_) | OrNotBetween(_) => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_simple() {
        let c = vec![AndEq("id")];
        assert_eq!(r#""id" = ?"#, Condition::gen_all(&c, TargetDatabase::Sqlite));
    }

    #[test]
    fn test_condition_more() {
        let c = vec![AndEq("id"), And(vec![AndEq("gender"), OrGe("iq")]), OrNotBetween("age")];
        let left = r#""id" = ? AND ("gender" = ? OR "iq" >= ?) OR "age" NOT BETWEEN ? AND ?"#;
        assert_eq!(left, Condition::gen_all(&c, TargetDatabase::Sqlite));
    }

}