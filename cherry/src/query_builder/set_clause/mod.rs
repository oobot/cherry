use crate::query_builder::set_clause::SetSection::*;
use crate::query_builder::TargetQuery::{self, *};

pub(crate) struct SetClause<'a> {
    target: TargetQuery,
    sections: Vec<SetSection<'a>>,
}

impl<'a> SetClause<'a> {

    pub(crate) fn from(target: TargetQuery) -> Self {
        SetClause { target, sections: vec![] }
    }

    pub(crate) fn add(&mut self, s: SetSection<'a>) {
        self.sections.push(s);
    }

    pub(crate) fn as_clause(&self) -> Option<String> {
        let clause = self.sections.iter().map(|s| {
            match s {
                SetValue(c) => format!("{} = ?", self.target.quote(c)),
                SetColumn(c) => match self.target {
                    // https://dev.mysql.com/doc/refman/8.0/en/insert-on-duplicate.html
                    MySql => format!(r#"{0} = new.{0}"#, self.target.quote(c)),
                    Postgres | Sqlite => format!(r#"{0} = excluded.{0}"#, self.target.quote(c)),
                }
            }
        }).collect::<Vec<String>>().join(", ");

        match clause.is_empty() {
            true => None,
            _ => Some(clause),
        }
    }

}

pub enum SetSection<'a> {
    SetValue(&'a str),
    SetColumn(&'a str),
}