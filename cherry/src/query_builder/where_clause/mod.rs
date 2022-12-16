use crate::query_builder::TargetQuery;
use crate::query_builder::where_clause::condition::Condition;

pub mod condition;

pub struct WhereClause<'a> {
    target: TargetQuery,
    conditions: Vec<Condition<'a>>,
    temp_conditions: Option<Vec<Condition<'a>>>,
}

impl<'a> WhereClause<'a> {

    pub fn from(target: TargetQuery) -> Self {
        Self {
            target,
            conditions: vec![],
            temp_conditions: None,
        }
    }

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

    pub fn as_sql(&self) -> Option<String> {
        match self.conditions.is_empty() {
            true => None,
            _ => Some(Condition::gen_all(&self.conditions, self.target))
        }
    }

}