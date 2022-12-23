use crate::sql::end_clause::EndSection::*;
use crate::sql::TargetDatabase;

#[derive(Default)]
pub struct EndClause<'a> {
    sections: Vec<EndSection<'a>>
}

impl<'a> EndClause<'a> {

    pub fn add(&mut self, section: EndSection<'a>) {
        self.sections.push(section);
    }

    pub fn as_clause(&self, db: TargetDatabase) -> Option<String> {
        match self.sections.is_empty() {
            true => None,
            _ => Some(Self::gen_sections(db, &self.sections))
        }
    }

    fn gen_sections(_db: TargetDatabase, sections: &[EndSection]) -> String {
        sections.iter()
            .map(|v| v.as_statement())
            .collect::<Vec<String>>()
            .join(" ")
    }

}


pub enum EndSection<'a> {
    OrderBy(&'a str, bool), // column, asc or desc
    Limit(),
    Offset(),
}

impl<'a> EndSection<'a> {
    pub fn as_statement(&self) -> String {
        match &self {
            OrderBy(c, asc) => match *asc {
                true => format!("ORDER BY {} ASC", c),
                false => format!("ORDER BY {} DESC", c),
            }
            Limit() => "LIMIT ?".into(),
            Offset() => "OFFSET ?".into(),
        }
    }

}




#[cfg(test)]
mod tests {
    use crate::sql::end_section::EndSection::{Limit, Offset, OrderBy};

    use super::*;

    #[test]
    fn test_ending_simple() {
        let c  = vec![OrderBy("name", false)];
        assert_eq!("ORDER BY name DESC", EndClause::gen_sections(TargetDatabase::Sqlite, &c));
    }

    #[test]
    fn test_ending_more() {
        let c  = vec![OrderBy("name", false), Limit(), Offset()];
        assert_eq!("ORDER BY name DESC LIMIT ? OFFSET ?", EndClause::gen_sections(TargetDatabase::Sqlite, &c));
    }
}
