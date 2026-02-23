use std::{
    fmt::{Display, Formatter as FmtFormatter, Result as FmtResult},
    result::Result as StdResult,
};

/// Custom result type to encapsulate everything that happens in here.
pub type StagResult<T, E = StagError> = StdResult<T, E>;

/// Error coalescence enumeration, meant to be used with the custom result type.
pub enum StagError {
    // entity-related errors
    EntityAttributeNotFound(String),
    EntityWrongAttributeOverride(String),
    // graph-related errors
    NonexistantLink(String),
    UnauthorizedLinkOverride,
}

// Display error definition for easy reading
impl Display for StagError {
    fn fmt(&self, frm: &mut FmtFormatter<'_>) -> FmtResult {
        match self {
            Self::EntityAttributeNotFound(name) => {
                frm.write_str(format!("Attribute {} not found", name).as_str())
            }
            Self::EntityWrongAttributeOverride(name) => {
                frm.write_str(format!("Can't change attribute {}", name).as_str())
            }
            Self::NonexistantLink(name) => {
                frm.write_str(format!("Nonexistant link {}", name).as_str())
            }
            Self::UnauthorizedLinkOverride => {
                frm.write_str("Unauthorized graph link value override")
            }
        }
    }
}
