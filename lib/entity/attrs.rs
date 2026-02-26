use regex::Regex;

use crate::errors::StagError;

// some constants for simplicity
const BOOLEAN_NAME: &'static str = "bool";
const INT_NAME: &'static str = "int";
const UUID_NAME: &'static str = "uuid";
const TEXT_NAME: &'static str = "text";
const VARCHAR_NAME: &'static str = "varchar";
const CHAR_NAME: &'static str = "char";

/// Enumeration for every possible attribute type available to an SGBD.
/// Each conversion core should exclude the ones it does not want.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntityAttr {
    BOOLEAN,
    // numbers
    INTEGER,
    // text
    TEXT,
    CHAR(usize),
    VARCHAR(usize),
    // identifiers
    UUID,
}

impl Default for EntityAttr {
    fn default() -> Self {
        Self::INTEGER
    }
}

impl ToString for EntityAttr {
    fn to_string(&self) -> String {
        match self {
            Self::BOOLEAN => BOOLEAN_NAME.to_string(),
            Self::INTEGER => INT_NAME.to_string(),
            Self::TEXT => TEXT_NAME.to_string(),
            Self::CHAR(n) => format!("{}({})", CHAR_NAME, n),
            Self::VARCHAR(n) => format!("{}({})", VARCHAR_NAME, n),
            Self::UUID => UUID_NAME.to_string(),
        }
    }
}

impl TryFrom<String> for EntityAttr {
    type Error = StagError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            BOOLEAN_NAME => Ok(Self::BOOLEAN),
            INT_NAME => Ok(Self::INTEGER),
            TEXT_NAME => Ok(Self::TEXT),
            UUID_NAME => Ok(Self::UUID),
            _ => match Regex::new(r"varchar([0-9]+)").unwrap().captures(&value) {
                Some(val) => {
                    let test: [&str; 1] = val.extract().1;
                    todo!()
                }
                _ => Err(StagError::ParseError),
            },
        }
    }
}
