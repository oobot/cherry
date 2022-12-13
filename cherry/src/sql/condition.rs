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

    AndIsNull(&'a str),
    OrIsNull(&'a str),

    AndBetween(&'a str),
    AndNotBetween(&'a str),
    OrBetween(&'a str),
    OrNotBetween(&'a str),
}

impl<'a> Condition<'a> {
    pub fn as_sql(&self) -> String {
        match &self {
            Condition::And(v) => {}
            Condition::Or(_) => {}
            Condition::AndEq(_) => {}
            Condition::OrEq(_) => {}
            Condition::AndGe(_) => {}
            Condition::OrGe(_) => {}
            Condition::AndGt(_) => {}
            Condition::OrGt(_) => {}
            Condition::AndLe(_) => {}
            Condition::OrLe(_) => {}
            Condition::AndLt(_) => {}
            Condition::OrLt(_) => {}
            Condition::AndIn(_, _) => {}
            Condition::OrIn(_, _) => {}
            Condition::AndIsNull(_) => {}
            Condition::OrIsNull(_) => {}
            Condition::AndBetween(_) => {}
            Condition::AndNotBetween(_) => {}
            Condition::OrBetween(_) => {}
            Condition::OrNotBetween(_) => {}
        }

        todo!()
    }
}


pub enum Ending<'a> {
    OrderBy(&'a str, bool), // column, asc or desc
    Limit(usize),
    Offset(usize),
}