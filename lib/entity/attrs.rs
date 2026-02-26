use crate::errors::StagError;

// some constants for simplicity
const BOOLEAN_NAME: &'static str = "bool";
const INT_NAME: &'static str = "int";

/// Enumeration for every possible attribute type available to an SGBD.
/// Each conversion core should exclude the ones it does not want.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntityAttr {
    BOOLEAN,
    // numbers
    INTEGER,
}

impl Default for EntityAttr {
    fn default() -> Self {
        Self::INTEGER
    }
}

impl ToString for EntityAttr {
    fn to_string(&self) -> String {
        format!(
            "{}",
            match self {
                Self::BOOLEAN => BOOLEAN_NAME,
                Self::INTEGER => INT_NAME,
            }
        )
    }
}

impl TryFrom<String> for EntityAttr {
    type Error = StagError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            BOOLEAN_NAME => Ok(Self::BOOLEAN),
            INT_NAME => Ok(Self::INTEGER),
            _ => Err(StagError::ParseError),
        }
    }
}
