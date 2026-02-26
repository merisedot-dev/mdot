use std::{
    fmt::{Display, Formatter as FmtFormatter, Result as FmtResult},
    result::Result as StdResult,
};

/// Custom result type to encapsulate everything that happens in here.
pub type StagResult<T, E = StagError> = StdResult<T, E>;

/// Error coalescence enumeration, meant to be used with the custom result type.
#[derive(Debug)]
pub enum StagError {
    // entity-related errors
    EntityAttributeNotFound(String),
    EntityWrongAttributeOverride(String),
    // graphlink-related errors
    NonexistantLink(String),
    UnauthorizedLinkOverride,
    // graph-related errors
    ExistingEntity(String),
    UnknownEntity(String),
    // parsing error
    ParseError,
}

// Display error definition for easy reading
impl Display for StagError {
    fn fmt(&self, frm: &mut FmtFormatter<'_>) -> FmtResult {
        match self {
            // entity errors
            Self::EntityAttributeNotFound(name) => {
                frm.write_str(format!("Attribute {} not found", name).as_str())
            }
            Self::EntityWrongAttributeOverride(name) => {
                frm.write_str(format!("Can't change attribute {}", name).as_str())
            }
            // graphlink errors
            Self::NonexistantLink(name) => {
                frm.write_str(format!("Nonexistant link {}", name).as_str())
            }
            Self::UnauthorizedLinkOverride => {
                frm.write_str("Unauthorized graph link value override")
            }
            // graph errors
            Self::ExistingEntity(name) => {
                frm.write_str(format!("Can't override entity {}", name).as_str())
            }
            Self::UnknownEntity(name) => {
                frm.write_str(format!("Entity {} doesn't exist", name).as_str())
            }
            Self::ParseError => frm.write_str("parse error"),
        }
    }
}
