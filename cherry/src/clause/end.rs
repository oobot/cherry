use sqlx::{Database, Encode, Type};

use crate::provider::Provider;
use crate::sql::end_clause::EndSection;

pub trait End<'a, DB>: Provider<'a, DB> + Sized where DB: Database {

    fn order_by_asc(mut self, c: &'a str) -> Self {
        self.sql_builder().add_end_section(EndSection::OrderBy(c, true));
        self
    }

    fn order_by_desc(mut self, c: &'a str) -> Self {
        self.sql_builder().add_end_section(EndSection::OrderBy(c, false));
        self
    }

    fn limit<V>(mut self, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.sql_builder().add_end_section(EndSection::Limit());
        self
    }

    fn offset<V>(mut self, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.sql_builder().add_end_section(EndSection::Offset());
        self
    }

}