use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Cardi {
    ANY(i8), // any defined number of linked elements
    ZERO,    // the foreign key is nullable
    MANY,    // undeterminate number of linked elements
}

impl Default for Cardi {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<i8> for Cardi {
    fn from(value: i8) -> Self {
        if value == 0 {
            Self::ZERO
        } else if value < 0 {
            Self::MANY
        } else {
            Self::ANY(value)
        }
    }
}

impl Display for Cardi {
    fn fmt(&self, frm: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ANY(val) => frm.write_str(format!("{}", val).as_str()),
            Self::ZERO => frm.write_str("0"),
            Self::MANY => frm.write_str("n"),
        }
    }
}
