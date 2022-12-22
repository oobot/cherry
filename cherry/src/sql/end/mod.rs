use crate::sql::end::section::EndSection;
use crate::sql::TargetQuery;

pub mod section;

pub struct EndClause<'a> {
    _target: TargetQuery,
    sections: Vec<EndSection<'a>>
}

impl<'a> EndClause<'a> {

    pub fn from(db: TargetQuery) -> Self {
        Self { _target: db, sections: vec![] }
    }

    pub fn add(&mut self, section: EndSection<'a>) {
        self.sections.push(section);
    }

    pub fn as_clause(&self) -> Option<String> {
        match self.sections.is_empty() {
            true => None,
            _ => Some(EndSection::gen_all(&self.sections))
        }
    }

}