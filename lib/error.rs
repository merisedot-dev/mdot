use std::fmt::{Display, Formatter as StdFormatter, Result as FmtResult};

pub type StagResult<T, E = StagError> = Result<T, E>;

#[derive(Debug)]
pub enum StagError {
    // Custom error causes
    AttributeNotFound(String),
    EntityNotFound,
    LinkNotFound,
    ExistingEntity(String),
    ExistingLink,
}

impl Display for StagError {
    fn fmt(&self, frm: &mut StdFormatter<'_>) -> FmtResult {
        match self {
            Self::AttributeNotFound(attr) => {
                frm.write_str(format!("Attr {} not found", attr).as_str())
            }
            Self::EntityNotFound => frm.write_str("Entity not found"),
            Self::LinkNotFound => frm.write_str("Link not found"),
            StagError::ExistingEntity(e) => {
                frm.write_str(format!("Already existing entity {}", e).as_str())
            }
            Self::ExistingLink => frm.write_str("Already existing link"),
        }
    }
}
