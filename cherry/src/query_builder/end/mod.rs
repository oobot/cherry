use crate::query_builder::end::section::EndSection;
use crate::query_builder::TargetDatabase;

pub mod section;

pub struct EndStatement<'a> {
    db: TargetDatabase,
    sections: Vec<EndSection<'a>>
}

impl<'a> EndStatement<'a> {

    pub fn from(db: TargetDatabase) -> Self {
        Self { db, sections: vec![] }
    }

    pub fn add(&mut self, section: EndSection<'a>) {
        self.sections.push(section);
    }

    pub fn as_statement(&self) -> Option<String> {
        match self.sections.is_empty() {
            true => None,
            _ => Some(EndSection::gen_all(&self.sections))
        }
    }

}
