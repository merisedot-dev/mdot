use crate::constraint::ForeignKeyConstraint;

/// SQL constraint logic model. This is made to be shared between targets,
/// however, for serialization purposes, dedicated converters will be used.
pub enum Constraint {
    ForeignKey(ForeignKeyConstraint),
    Unique,
}
