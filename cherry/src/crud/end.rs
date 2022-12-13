use sqlx::{Database, Encode, Type};

use crate::crud::provider::EndProvider;
use crate::statement::end::section::EndSection;

pub trait End<'a, DB>: EndProvider<'a, DB> + Sized where DB: Database {

    fn order_by_asc<V>(mut self, c: &'a str) -> Self {
        self.add_section(EndSection::OrderBy(c, true));
        self
    }

    fn order_by_desc(mut self, c: &'a str) -> Self {
        self.add_section(EndSection::OrderBy(c, false));
        self
    }

    fn limit<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_section(EndSection::Limit());
        self
    }

    fn offset<V>(mut self, c: &'a str, v: V) -> Self
        where V: Encode<'a, DB> + Type<DB> + Send + 'a {
        self.add_value(v);
        self.add_section(EndSection::Offset());
        self
    }

}