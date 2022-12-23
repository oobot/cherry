use crate::sql::update_set_clause::UpdateSetSection::*;
use crate::sql::TargetDatabase::{self, *};

#[derive(Default)]
pub(crate) struct UpdateSetClause<'a> {
    sections: Vec<UpdateSetSection<'a>>,
}

impl<'a> UpdateSetClause<'a> {

    pub(crate) fn add(&mut self, s: UpdateSetSection<'a>) {
        self.sections.push(s);
    }

    pub(crate) fn as_clause(&self, db: TargetDatabase) -> Option<String> {
        let clause = self.sections.iter().map(|s| {
            match s {
                SetValue(c) => format!("{} = ?", db.quote(c)),
                SetColumn(c) => match db {
                    // https://dev.mysql.com/doc/refman/8.0/en/insert-on-duplicate.html
                    MySql => format!(r#"{0} = new.{0}"#, db.quote(c)),
                    Postgres | Sqlite => format!(r#"{0} = excluded.{0}"#, db.quote(c)),
                }
            }
        }).collect::<Vec<String>>().join(", ");

        match clause.is_empty() {
            true => None,
            _ => Some(clause),
        }
    }

}

pub enum UpdateSetSection<'a> {
    SetValue(&'a str),
    SetColumn(&'a str),
}