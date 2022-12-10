use crate::Cherry;

pub struct Insert<'a, T: Cherry> {
    pub(crate) v: &'a T,
}

impl<'a, T: Cherry> Insert<'a, T> {
    pub fn from(v: &'a T) -> Self {
        Self { v }
    }

}