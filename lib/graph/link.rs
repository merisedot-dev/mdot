use std::collections::HashMap;

use crate::{
    attribute::EntityAttribute,
    constants::DEFAULT_CARDINALITIES,
    entity::Entity,
    error::{StagError, StagResult},
};

#[derive(Clone)]
pub struct GraphLink<T>
where
    T: EntityAttribute,
{
    entity: Entity<T>,
    cardinalities: HashMap<String, (i8, i8)>,
}

impl<T> GraphLink<T>
where
    T: EntityAttribute,
{
    pub fn new(name: impl ToString) -> Self {
        Self {
            entity: Entity::new(name),
            cardinalities: HashMap::new(),
        }
    }

    /// Adds a new linked [Entity] to the current [GraphLink]. If the given
    /// [Entity] is already linked, this method will throw a [StagError].
    pub fn link_entity(&mut self, entity: Entity<T>) -> StagResult<()> {
        if let Some(_) = self.cardinalities.get(&entity.get_name()) {
            // failsafe return
            return Err(StagError::ExistingLink);
        }
        self.cardinalities
            .insert(entity.get_name(), DEFAULT_CARDINALITIES);
        Ok(())
    }
}
