use crate::sql::TargetDatabase;
use crate::sql::where_condition::Condition;

#[derive(Default)]
pub struct WhereClause<'a> {
    conditions: Vec<Condition<'a>>,
    temp_conditions: Option<Vec<Condition<'a>>>,
}

impl<'a> WhereClause<'a> {

    pub fn add(&mut self, condition: Condition<'a>) {
        match &mut self.temp_conditions {
            Some(vec) => vec.push(condition),
            _ => self.conditions.push(condition),
        }
    }

    pub fn make_temp(&mut self) {
        self.temp_conditions = Some(vec![]);
    }

    pub fn take_temp(&mut self) -> Vec<Condition<'a>> {
        self.temp_conditions.take().unwrap_or_default()
    }

    pub fn as_sql(&self, db: TargetDatabase) -> Option<String> {
        match self.conditions.is_empty() {
            true => None,
            _ => Some(Self::gen_conditions(db, &self.conditions)),
        }
    }

    fn gen_conditions(db: TargetDatabase, conditions: &[Condition]) -> String {
        conditions.iter().enumerate().map(|(i, condition)| {
            match i {
                0 => Self::gen_one(db, condition),
                _ => match condition.and_or() {
                    true => format!(" AND {}", Self::gen_one(db, condition)),
                    _ => format!(" OR {}", Self::gen_one(db, condition)),
                }
            }
        }).collect::<String>()
    }

    fn gen_one(db: TargetDatabase, condition: &Condition) -> String {
        match condition {
            Condition::And(c) => format!("({})", Self::gen_conditions(db, c)),
            Condition::Or(c) => format!("({})", Self::gen_conditions(db, c)),
            _ => condition.as_sql(db),
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::sql::where_condition::Condition::{And, AndEq, OrGe, OrNotBetween};

    use super::*;

    #[test]
    fn test_condition_simple() {
        let c = vec![AndEq("id")];
        assert_eq!(r#""id" = ?"#, WhereClause::gen_conditions(TargetDatabase::Sqlite, &c));
    }

    #[test]
    fn test_condition_more() {
        let c = vec![AndEq("id"), And(vec![AndEq("gender"), OrGe("iq")]), OrNotBetween("age")];
        let left = r#""id" = ? AND ("gender" = ? OR "iq" >= ?) OR "age" NOT BETWEEN ? AND ?"#;
        assert_eq!(left, WhereClause::gen_conditions(TargetDatabase::Sqlite, &c));
    }

}