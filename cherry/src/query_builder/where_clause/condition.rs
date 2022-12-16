use crate::query_builder::TargetQuery;
use crate::query_builder::where_clause::condition::Condition::*;

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
    AndIsNotNull(&'a str),
    OrIsNull(&'a str),
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
    AndColumnIsNotNull(&'a str),
    OrColumnIsNull(&'a str),
    OrColumnIsNotNull(&'a str),
}

impl<'a> Condition<'a> {
    pub fn gen_all<'b>(vec: &'b [Condition<'b>], target: TargetQuery) -> String {
        vec.iter().enumerate().map(|(i, c)| {
            match i {
                0 => c.as_sql(target),
                _ => match c.and_or() {
                    true => format!(" AND {}", c.as_sql(target)),
                    _ => format!(" OR {}", c.as_sql(target)),
                }
            }
        }).collect::<String>()
    }

    fn as_sql(&self, target: TargetQuery) -> String {
        match &self {
            And(v) => format!("({})", Self::gen_all(v, target)),
            Or(v) => format!("({})", Self::gen_all(v, target)),
            AndEq(c) => format!("{} = ?", target.quote(c)),
            OrEq(c) => format!("{} = ?", target.quote(c)),
            AndGe(c) => format!("{} >= ?", target.quote(c)),
            OrGe(c) => format!("{} >= ?", target.quote(c)),
            AndGt(c) => format!("{} > ?", target.quote(c)),
            OrGt(c) => format!("{} > ?", target.quote(c)),
            AndLe(c) => format!("{} =< ?", target.quote(c)),
            OrLe(c) => format!("{} =< ?", target.quote(c)),
            AndLt(c) => format!("{} < ?", target.quote(c)),
            OrLt(c) => format!("{} < ?", target.quote(c)),
            AndIsNull(c) => format!("{} IS NULL", target.quote(c)),
            AndIsNotNull(c) => format!("{} IS NOT NULL", target.quote(c)),
            OrIsNull(c) => format!("{} IS NULL", target.quote(c)),
            OrIsNotNull(c) => format!("{} IS NOT NULL", target.quote(c)),
            AndBetween(c) => format!("{} BETWEEN ? AND ?", target.quote(c)),
            OrBetween(c) => format!("{} BETWEEN ? AND ?", target.quote(c)),
            AndNotBetween(c) => format!("{} NOT BETWEEN ? AND ?", target.quote(c)),
            OrNotBetween(c) => format!("{} NOT BETWEEN ? AND ?", target.quote(c)),
            AndIn(c, n) => format!("{} IN ({})", target.quote(c), vec!["?"; *n].join(", ")),
            AndNotIn(c, n) => format!("{} NOT IN ({})", target.quote(c), vec!["?"; *n].join(", ")),
            OrIn(c, n) => format!("{} IN ({})", target.quote(c), vec!["?"; *n].join(", ")),
            OrNotIn(c, n) => format!("{} NOT IN ({})", target.quote(c), vec!["?"; *n].join(", ")),

            AndEqColumn(_) => format!(""),
            OrEqColumn(_) => format!(""),
            AndGeColumn(_) => format!(""),
            OrGeColumn(_) => format!(""),
            AndGtColumn(_) => format!(""),
            OrGtColumn(_) => format!(""),
            AndLeColumn(_) => format!(""),
            OrLeColumn(_) => format!(""),
            AndLtColumn(_) => format!(""),
            OrLtColumn(_) => format!(""),
            AndColumnIsNull(_) => format!(""),
            AndColumnIsNotNull(_) => format!(""),
            OrColumnIsNull(_) => format!(""),
            OrColumnIsNotNull(_) => format!(""),
        }
    }

    fn and_or(&self) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_simple() {
        let c = vec![AndEq("id")];
        assert_eq!(r#""id" = ?"#, Condition::gen_all(&c, TargetQuery::Sqlite));
    }

    #[test]
    fn test_condition_more() {
        let c = vec![AndEq("id"), And(vec![AndEq("gender"), OrGe("iq")]), OrNotBetween("age")];
        let left = r#""id" = ? AND ("gender" = ? OR "iq" >= ?) OR "age" NOT BETWEEN ? AND ?"#;
        assert_eq!(left, Condition::gen_all(&c, TargetQuery::Sqlite));
    }

}