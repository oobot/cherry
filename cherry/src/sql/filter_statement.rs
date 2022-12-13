use crate::sql::condition::{Condition, Ending};

pub struct FilterStatement<'a> {
    conditions: Vec<Condition<'a>>,
    temp_conditions: Option<Vec<Condition<'a>>>,
    ending: Vec<Ending<'a>>,
}

impl<'a> FilterStatement<'a> {

    pub fn new() -> Self {
        Self {
            conditions: vec![],
            temp_conditions: None,
            ending: vec![],
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

    pub fn end_with(&mut self, ending: Ending<'a>) {
        self.ending.push(ending);
    }

    pub fn as_statement(&self) -> Option<String> {
        let mut vec = Vec::with_capacity(2);
        if !self.conditions.is_empty() {
            vec.push(Condition::gen_all(&self.conditions));
        }
        if !self.ending.is_empty() {
            vec.push(Ending::gen_all(&self.ending));
        }
        match vec.is_empty() {
            true => None,
            _ => Some(vec.join(" ")),
        }
    }

}