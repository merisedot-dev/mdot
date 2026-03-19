use indexmap::IndexMap;

use crate::{
    constants::DEFAULT_CARDINALITY,
    entity::{Cardi, Entity},
    errors::{StagError, StagResult},
};

#[derive(Clone, Debug, Default)]
pub struct GraphLink {
    pub inner: Entity,
    lks: IndexMap<String, (String, Cardi, Cardi)>,
}

impl GraphLink {
    /// Define new [GraphLink], as well as its inner [Entity]. Informations such
    /// as the name or specific [GraphLink] attributes will be handled by the
    /// inner [Entity].
    pub fn new(name: impl ToString) -> Self {
        Self {
            inner: Entity::new(name),
            lks: IndexMap::new(),
        }
    }

    pub fn get_lks(&self) -> IndexMap<String, (String, Cardi, Cardi)> {
        self.lks.clone()
    }

    /// Fetches role and cardinalities informations from [GraphLink]. Role can
    /// be empty depending on [GraphLink] usage (such as MLD graph).
    ///
    /// **Warning**: In case of a nonexistant link, it will throw a
    /// [StagError::NonexistantLink] error.
    pub fn get_entity_link(&self, name: impl ToString) -> StagResult<&(String, Cardi, Cardi)> {
        let str_name = name.to_string().to_lowercase();
        match self.lks.get(&str_name) {
            Some(val) => Ok(val),
            _ => Err(StagError::NonexistantLink(str_name)),
        }
    }

    /// Adds a new link to the current [GraphLink]. Cardinalities and role are set
    /// to default value at first, please call the [GraphLink::set_role] and
    /// [GraphLink::set_cardinality] methods to change said values.
    ///
    /// **Warning**: In case of an already known [Entity], it will throw a
    /// [StagError::UnauthorizedLinkOverride] error.
    pub fn link_to(&mut self, entity: Entity) -> StagResult<()> {
        let str_name = entity.name();
        if self.lks.contains_key(&str_name) {
            Err(StagError::UnauthorizedLinkOverride)
        } else {
            self.lks.insert(str_name, DEFAULT_CARDINALITY);
            Ok(())
        }
    }

    /// Changes the known role of a given link in the current [GraphLink]. The
    /// [Entity] is there for identification purposes. This will not change any
    /// existing cardinality.
    ///
    /// **Warning**: In case of an unknown entity, throws a
    /// [StagError::NonexistantLink] error.
    pub fn set_role(&mut self, e: Entity, r: impl ToString) -> StagResult<()> {
        if let Some((_, min, max)) = self.lks.get(&e.name()) {
            self.lks
                .insert(e.name(), (r.to_string(), min.clone(), max.clone()));
            Ok(())
        } else {
            Err(StagError::NonexistantLink(e.name()))
        }
    }

    /// Changes existing cardinalities for a given [Entity] in the current
    /// [GraphLink]. This will not edit any known role, even an empty one.
    ///
    /// **Warning**: In case of an unknown entity, throws a
    /// [StagError::NonexistantLink] error.
    pub fn set_cardinality(&mut self, e: impl ToString, n: Cardi, m: Cardi) -> StagResult<()> {
        let str_name = e.to_string().to_lowercase();
        if let Some((role, _, _)) = self.lks.get(&str_name) {
            self.lks.insert(str_name, (role.clone(), n, m));
            Ok(())
        } else {
            Err(StagError::NonexistantLink(str_name))
        }
    }

    /// Deletes a link to a given [Entity] from the current [GraphLink]. This
    /// will not touch the entity itself, just informations known about its link.
    ///
    /// **Warning**: In case of an unknown link, throws a
    /// [StagError::NonexistantLink] error.
    pub fn unlink(&mut self, entity: Entity) -> StagResult<()> {
        let str_name = entity.name();
        if self.lks.contains_key(&str_name) {
            self.lks.shift_remove(&str_name);
            Ok(())
        } else {
            Err(StagError::NonexistantLink(str_name))
        }
    }
}
