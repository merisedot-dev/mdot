use std::collections::HashMap;

use crate::{
    entity::{Entity, GraphLink},
    errors::{StagError, StagResult},
};

#[derive(Clone, Default)]
pub struct Graph {
    entities: HashMap<String, Entity>,
    links: HashMap<String, GraphLink>,
}

impl Graph {
    /// Builds a new [Entity] in the current [Graph]. The newly built [Entity]
    /// will be devoid of any data. Since the name will act as ID, it will be
    /// lowercased.
    ///
    /// **Warning**: In case of already exixting [Entity], throws a
    /// [StagError::ExistingEntity] error.
    pub fn mk_entity(&mut self, name: impl ToString) -> StagResult<&Entity> {
        let str_name = name.to_string().to_lowercase();
        if self.entities.contains_key(&str_name) {
            Err(StagError::ExistingEntity(str_name))
        } else {
            self.entities
                .insert(str_name.clone(), Entity::new(str_name.clone()));
            Ok(self.entities.get(&str_name).unwrap())
        }
    }

    /// Removes an [Entity] from the current [Graph], as well as any reference
    /// to it stored by the [GraphLink]s.
    ///
    /// **Warning**: In case of the entity not being found, throws a
    /// [StagError::UnknownEntity] error.
    pub fn del_entity(&mut self, name: impl ToString) -> StagResult<()> {
        let str_name = name.to_string().to_lowercase();
        // failsafe check
        if !self.entities.contains_key(&str_name) {
            return Err(StagError::UnknownEntity(str_name));
        }
        let entity = self.entities.remove(&str_name).unwrap(); // we just checked
        for (_, lk) in self.links.iter_mut() {
            let _ = lk.unlink(entity.clone()); // the error isn't important here
        }
        Ok(())
    }

    /// Fetches relevant [Entity] from the current [Graph]. This also exposes
    /// the pointer to ensure further modifications can be made to it.
    ///
    /// **Warning**: In case of the [Entity] not being in graph, throws a
    /// [StagError::UnknownEntity] error.
    pub fn get_entity(&self, name: impl ToString) -> StagResult<&Entity> {
        let str_name = name.to_string().to_lowercase();
        match self.entities.get(&str_name) {
            Some(val) => Ok(val),
            None => Err(StagError::UnknownEntity(str_name)),
        }
    }
}
