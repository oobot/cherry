use crate::statement::r#where::condition::Condition::*;

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
    pub fn as_statement(&self) -> String {
        match &self {
            And(v) => format!("({})", Self::gen_all(v)),
            Or(v) => format!("({})", Self::gen_all(v)),
            AndEq(c) => format!("{} = ?", c),
            OrEq(c) => format!("{} = ?", c),
            AndGe(c) => format!("{} >= ?", c),
            OrGe(c) => format!("{} >= ?", c),
            AndGt(c) => format!("{} > ?", c),
            OrGt(c) => format!("{} > ?", c),
            AndLe(c) => format!("{} =< ?", c),
            OrLe(c) => format!("{} =< ?", c),
            AndLt(c) => format!("{} < ?", c),
            OrLt(c) => format!("{} < ?", c),
            AndIn(c, n) => format!("{} IN ({})", c, vec!["?"; *n].join(", ")),
            AndNotIn(c, n) => format!("{} NOT IN ({})", c, vec!["?"; *n].join(", ")),
            OrIn(c, n) => format!("{} IN ({})", c, vec!["?"; *n].join(", ")),
            OrNotIn(c, n) => format!("{} NOT IN ({})", c, vec!["?"; *n].join(", ")),
            AndIsNull(c) => format!("{} is null", c),
            OrIsNull(c) => format!("{} is not null", c),
            AndBetween(c) => format!("{} BETWEEN ? AND ?", c),
            OrBetween(c) => format!("{} BETWEEN ? AND ?", c),
            AndNotBetween(c) => format!("{} NOT BETWEEN ? AND ?", c),
            OrNotBetween(c) => format!("{} NOT BETWEEN ? AND ?", c),
        }
    }

    pub fn gen_all<'b>(vec: &'b [Condition<'b>]) -> String {
        vec.iter().enumerate().map(|(i, c)| {
            match i {
                0 => c.as_statement(),
                _ => match c.and_or() {
                    true => format!(" AND {}", c.as_statement()),
                    _ => format!(" OR {}", c.as_statement()),
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
        assert_eq!("id = ?", Condition::gen_all(&c));
    }

    #[test]
    fn test_condition_more() {
        let c = vec![AndEq("id"), And(vec![AndEq("gender"), OrGe("iq")]), OrNotBetween("age")];
        let left = "id = ? AND (gender = ? OR iq >= ?) OR age NOT BETWEEN ? AND ?";
        assert_eq!(left, Condition::gen_all(&c));
    }

}