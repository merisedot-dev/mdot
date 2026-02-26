use crate::errors::StagError;

// some constants for simplicity
const BOOLEAN_NAME: &'static str = "bool";
const INT_NAME: &'static str = "int";
const UUID_NAME: &'static str = "uuid";
const TEXT_NAME: &'static str = "text";
const VARCHAR_NAME: &'static str = "varchar";

/// Enumeration for every possible attribute type available to an SGBD.
/// Each conversion core should exclude the ones it does not want.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntityAttr {
    BOOLEAN,
    // numbers
    INTEGER,
    // text
    TEXT,
    VARCHAR(u64),
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
            // TODO regex for varchar
            UUID_NAME => Ok(Self::UUID),
            _ => Err(StagError::ParseError),
        }
    }
}
