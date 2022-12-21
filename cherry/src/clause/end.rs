use sqlx::{Database, Encode, Type};

use crate::provider::EndProvider;
use crate::sql::end::section::EndSection;

pub trait End<'a, DB>: EndProvider<'a, DB> + Sized where DB: Database {

    fn order_by_asc<V>(mut self, c: &'a str) -> Self {
        self.add_end_section(EndSection::OrderBy(c, true));
        self
    }

    fn order_by_desc(mut self, c: &'a str) -> Self {
        self.add_end_section(EndSection::OrderBy(c, false));
        self
    }

    fn limit<V>(mut self, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_end_section(EndSection::Limit());
        self
    }

    fn offset<V>(mut self, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_end_section(EndSection::Offset());
        self
    }

}