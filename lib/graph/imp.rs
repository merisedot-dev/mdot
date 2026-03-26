use std::collections::HashMap;

use crate::{
    entity::{Entity, GraphLink},
    errors::{StagError, StagResult},
};

#[derive(Clone, Debug, Default)]
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
    pub fn mk_entity(&mut self, name: impl ToString) -> StagResult<()> {
        let str_name = name.to_string().to_lowercase();
        if self.entities.contains_key(&str_name) {
            Err(StagError::ExistingEntity(str_name))
        } else {
            self.entities
                .insert(str_name.clone(), Entity::new(str_name.clone()));
            Ok(())
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

    /// Removes a given [GraphLink] from the current [Graph]. Since the name acts
    /// as ID, it will be lowercased.
    ///
    /// **Warning**: In case of missing [GraphLink], throws a
    /// [StagError::NonexistantLink] error back to caller.
    pub fn del_lk(&mut self, name: impl ToString) -> StagResult<()> {
        let str_name = name.to_string().to_lowercase();
        match self.links.remove(&str_name) {
            Some(_) => Ok(()),
            None => Err(StagError::NonexistantLink(str_name)),
        }
    }

    /// Fetches relevant [Entity] from the current [Graph].
    ///
    /// **Warning**: In case of the [Entity] not being in graph, throws a
    /// [StagError::UnknownEntity] error.
    pub fn get_ent(&self, name: impl ToString) -> StagResult<&Entity> {
        let str_name = name.to_string().to_lowercase();
        match self.entities.get(&str_name) {
            Some(val) => Ok(val),
            None => Err(StagError::UnknownEntity(str_name)),
        }
    }

    /// Fetches relevant [Entity] from the current [Graph], in mutable form.
    ///
    /// **Warning**: In case of the [Entity] not being in graph, throws a
    /// [StagError::UnknownEntity] error.
    pub fn edt_ent(&mut self, name: impl ToString) -> StagResult<&mut Entity> {
        let str_name = name.to_string().to_lowercase();
        match self.entities.get_mut(&str_name) {
            Some(val) => Ok(val),
            None => Err(StagError::UnknownEntity(str_name)),
        }
    }

    /// Fetches a [GraphLink] for edition in the current [Graph]. Do not use this
    /// if you only intend on looking in the [GraphLink].
    ///
    /// **Warning**: In case of missing [GraphLink], throws a
    /// [StagError::NonexistantLink] error back at the caller.
    pub fn edt_link(&mut self, lk: impl ToString) -> StagResult<&mut GraphLink> {
        let str_name = lk.to_string().to_lowercase();
        match self.links.get_mut(&str_name) {
            Some(val) => Ok(val),
            None => Err(StagError::NonexistantLink(str_name)),
        }
    }

    pub fn get_entities(&self) -> HashMap<String, Entity> {
        self.entities.clone()
    }

    pub fn get_lks(&self) -> HashMap<String, GraphLink> {
        self.links.clone()
    }

    /// Link two distinct entities together in the current [Graph]. As usual,
    /// all names will be lowercased.
    ///
    /// **Warning**: In case of a missing entity, throws a
    /// [StagError::UnknownEntity] back to caller.
    pub fn link(
        &mut self,
        name: impl ToString,
        entity1_name: impl ToString,
        entity2_name: impl ToString,
    ) -> StagResult<()> {
        let str_name = name.to_string().to_lowercase();
        let name1 = entity1_name.to_string().to_lowercase();
        let name2 = entity2_name.to_string().to_lowercase();
        // building link
        if let (Some(entity1), Some(entity2)) =
            (self.entities.get(&name1), self.entities.get(&name2))
        {
            let mut graph_link = GraphLink::new(str_name.clone());
            graph_link.link_to(entity1.clone())?;
            graph_link.link_to(entity2.clone())?;
            self.links.insert(str_name, graph_link);
            Ok(())
        } else {
            Err(StagError::UnknownEntity(format!("{} and {}", name1, name2)))
        }
    }

    /// Adds a new [Entity] to an existing [GraphLink] in the current [Graph].
    ///
    /// **Warning**: In case of a non-existant [GraphLink] or [Entity], throws a
    /// [StagError::NonexistantLink] or [StagError::UnknownEntity] respectively.
    pub fn extra_lk(
        &mut self,
        graphlink_name: impl ToString,
        entity_name: impl ToString,
    ) -> StagResult<()> {
        let glk_name = graphlink_name.to_string().to_lowercase();
        let e_name = entity_name.to_string().to_lowercase();
        // fetching from graph
        match (self.links.get_mut(&glk_name), self.entities.get(&e_name)) {
            (Some(glk), Some(e)) => {
                glk.link_to(e.clone())?;
                Ok(())
            }
            (_, None) => Err(StagError::UnknownEntity(e_name)),
            (None, _) => Err(StagError::NonexistantLink(glk_name)),
        }
    }

    /// Fetches a [GraphLink] from the current [Graph]. Since the name acts as ID,
    /// it will be lowercased.
    ///
    /// **Warning**: In case of unknown link, throws a
    /// [StagError::NonexistantLink] error.
    pub fn get_lk(&self, name: impl ToString) -> StagResult<&GraphLink> {
        let str_name = name.to_string().to_lowercase();
        match self.links.get(&str_name) {
            Some(val) => Ok(val),
            None => Err(StagError::NonexistantLink(str_name)),
        }
    }
}
