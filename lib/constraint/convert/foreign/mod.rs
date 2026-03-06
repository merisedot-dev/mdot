use crate::{
    entity::Entity,
    errors::{StagError, StagResult},
};

pub struct ForeignKeyConstraint {
    entity: Entity,
    target_attr: String,
    reference: Entity,
}

impl ForeignKeyConstraint {
    /// Builds a new [ForeignKeyConstraint] that makes sense. In case both
    /// [Entity] are the same one, throws a [StagError::IdenticalEntities] error.
    pub fn new(
        target_attr_name: impl ToString,
        entity: Entity,
        reference: Entity,
    ) -> StagResult<Self> {
        if entity == reference {
            return Err(StagError::IdenticalEntities(entity));
        }
        Ok(Self {
            entity,
            target_attr: target_attr_name.to_string(),
            reference,
        })
    }

    /// The [Entity] the [ForeignKeyConstraint] will apply on.
    pub fn entity(&self) -> Entity {
        self.entity.clone()
    }

    /// The exact attribute of the constrained [Entity] that will be used as a
    /// foreign key. In case of mismatch, not our problem.
    pub fn target_attr(self) -> String {
        self.target_attr.clone()
    }

    /// The referred [Entity] in the [ForeignKeyConstraint]. This is not the
    /// [Entity] the constraint will be applied to.
    pub fn reference(&self) -> Entity {
        self.reference.clone()
    }
}
