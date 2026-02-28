use std::fmt::{Display, Formatter};

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

impl Display for EntityAttr {
    fn fmt(&self, frm: &mut Formatter<'_>) -> std::fmt::Result {
        let st = match self {
            Self::BOOLEAN => BOOLEAN_NAME.to_string(),
            Self::INTEGER => INT_NAME.to_string(),
            Self::TEXT => TEXT_NAME.to_string(),
            Self::CHAR(n) => format!("{}({})", CHAR_NAME, n),
            Self::VARCHAR(n) => format!("{}({})", VARCHAR_NAME, n),
            Self::UUID => UUID_NAME.to_string(),
        };
        // actual formatting
        frm.write_str(st.as_str())
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
            _ => {
                if let Some(varchar_val) = value
                    .clone()
                    .strip_prefix(format!("{}(", VARCHAR_NAME).as_str())
                {
                    Ok(Self::VARCHAR(
                        varchar_val
                            .strip_suffix(")")
                            .unwrap_or_default()
                            .parse::<usize>()?,
                    ))
                } else if let Some(char_val) = value
                    .clone()
                    .strip_prefix(format!("{}(", CHAR_NAME).as_str())
                {
                    Ok(Self::CHAR(
                        char_val
                            .strip_suffix(")")
                            .unwrap_or_default()
                            .parse::<usize>()?,
                    ))
                } else {
                    // default scenario, nothing works
                    Err(StagError::ParseError)
                }
            }
        }
    }
}
