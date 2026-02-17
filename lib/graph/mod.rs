mod link;

use std::collections::HashMap;

use crate::{
    attribute::EntityAttribute,
    entity::Entity,
    error::{StagError, StagResult},
    graph::link::GraphLink,
};

/// A single layer of a database graph.
#[derive(Clone)]
pub struct MeriseGraph<T>
where
    T: EntityAttribute,
{
    entities: HashMap<String, Entity<T>>, // the entities themselves
    links: HashMap<String, GraphLink<T>>, // link between two or more entities
}

impl<T> MeriseGraph<T>
where
    T: EntityAttribute,
{
    /// Create a new, empty [MeriseGraph], devoid of any [Entity] or link.
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            links: HashMap::new(),
        }
    }

    /// Add a new [Entity] to the current graph and give it back to the caller.
    /// In case of an already existing [Entity], the function will fail,
    /// returning a [StagError].
    pub fn mk_entity(&mut self, name: impl ToString) -> StagResult<&Entity<T>> {
        let str_name = name.to_string().to_lowercase();
        if let Some(_) = self.entities.get(&str_name) {
            // no overriding entities
            return Err(StagError::ExistingEntity(str_name.clone()));
        }
        self.entities
            .insert(str_name.clone(), Entity::new(str_name.clone()));
        // return newly formed entity
        self.get_entity(str_name)
    }

    /// Fetches relevant [Entity] from graph. Keep in mind the name of the
    /// entity will act as ID, thus the `to_lowercase` call.
    pub fn get_entity(&self, name: impl ToString) -> StagResult<&Entity<T>> {
        match self.entities.get(&name.to_string().to_lowercase()) {
            Some(val) => Ok(val),
            None => Err(StagError::EntityNotFound),
        }
    }

    /// Removes an existing [Entity] from graph, and remove each [GraphLink]
    /// previously linked to it. If any [GraphLink] is down to only one link,
    /// it will be removed from graph.
    ///
    /// **Note** : This may be a bit longer than expected for large databases...
    pub fn del_entity(&mut self, name: impl ToString) -> StagResult<()> {
        let entity = self.get_entity(name.to_string())?;
        self.del_entity(entity.get_name())?;
        for (_e_name, _link) in self.links.iter() {
            // TODO check if entity is in link entity
            // TODO delete said entity
            // TODO check link post-deletion
        }
        Ok(())
    }

    /// Fetches relevant [GraphLink] from graph. As for an [Entity], the name
    /// will act as ID, so they will be passed through `to_lowercase`.
    pub fn get_link(&self, name: impl ToString) -> StagResult<&GraphLink<T>> {
        match self.links.get(&name.to_string().to_lowercase()) {
            Some(val) => Ok(val),
            None => Err(StagError::LinkNotFound),
        }
    }
}
