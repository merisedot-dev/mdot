use crate::{entity::Entity, errors::StagResult};

pub struct ForeignKeyConstraint {
    entity: Entity,
    target_attr: String,
    reference: Entity,
}

impl ForeignKeyConstraint {
    /// Builds a new [ForeignKeyConstraint] that makes sense. In case both
    /// [Entity] are the same one, throws a [StagError].
    pub fn new(
        target_attr_name: impl ToString,
        entity: Entity,
        reference: Entity,
    ) -> StagResult<Self> {
        todo!()
    }
}
