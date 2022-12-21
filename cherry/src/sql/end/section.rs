use crate::sql::end::section::EndSection::*;

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

    pub fn gen_all<'b>(vec: &'b[EndSection<'b>]) -> String {
        vec.iter().map(|v| v.as_statement()).collect::<Vec<String>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ending_simple() {
        let c  = vec![OrderBy("name", false)];
        assert_eq!("ORDER BY name DESC", EndSection::gen_all(&c));
    }

    #[test]
    fn test_ending_more() {
        let c  = vec![OrderBy("name", false), Limit(), Offset()];
        assert_eq!("ORDER BY name DESC LIMIT ? OFFSET ?", EndSection::gen_all(&c));
    }
}