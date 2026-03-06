use crate::{
    constraint::SQLConstraint,
    entity::Entity,
    errors::{StagError, StagResult},
};

pub struct ForeignKeyConstraint {
    name: String,
    entity: Entity,
    target_attr: String,
    reference: Entity,
}

impl ForeignKeyConstraint {
    /// Builds a new [ForeignKeyConstraint] that makes sense. In case both
    /// [Entity] are the same one, throws a [StagError::IdenticalEntities] error.
    pub fn new(
        name: impl ToString,
        target_attr_name: impl ToString,
        entity: Entity,
        reference: Entity,
    ) -> StagResult<Self> {
        if entity == reference {
            return Err(StagError::IdenticalEntities(entity));
        }
        Ok(Self {
            name: name.to_string(),
            entity,
            target_attr: target_attr_name.to_string(),
            reference,
        })
    }
    /// The referred [Entity] in the [ForeignKeyConstraint]. This is not the
    /// [Entity] the constraint will be applied to.
    pub fn reference(&self) -> Entity {
        self.reference.clone()
    }
}

impl SQLConstraint for ForeignKeyConstraint {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn entity(&self) -> Entity {
        self.entity.clone()
    }

    fn target_attr(&self) -> String {
        self.target_attr.clone()
    }
}
