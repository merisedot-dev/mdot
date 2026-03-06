use crate::constraint::convert::foreign::ForeignKeyConstraint;

/// SQL constraint logic model. This is made to be shared between targets,
/// however, for serialization purposes, dedicated converters will be used.
pub enum ESQLConstraint {
    ForeignKey(ForeignKeyConstraint),
    Unique,
}
