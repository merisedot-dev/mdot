use crate::constraint::{convert::foreign::FKConstraint, unique::UniqueConstraint};

/// SQL constraint logic model. This is made to be shared between targets,
/// however, for serialization purposes, dedicated converters will be used.
#[derive(Clone)]
pub enum ESQLConstraint {
    ForeignKey(FKConstraint),
    Unique(UniqueConstraint),
}
