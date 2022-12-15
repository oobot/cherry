use crate::query_builder::r#where::condition::Condition;

pub mod condition;

pub struct WhereStatement<'a> {
    conditions: Vec<Condition<'a>>,
    temp_conditions: Option<Vec<Condition<'a>>>,
}

impl<'a> WhereStatement<'a> {

    pub fn new() -> Self {
        Self {
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

    pub fn as_statement(&self) -> Option<String> {
        match self.conditions.is_empty() {
            true => None,
            _ => Some(Condition::gen_all(&self.conditions))
        }
    }

}