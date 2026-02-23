use std::collections::HashMap;

use crate::{
    entity::Entity,
    errors::{StagError, StagResult},
};

#[derive(Clone)]
pub struct GraphLink {
    inner: Entity,
    lks: HashMap<String, (String, u8, u8)>,
}

impl GraphLink {
    pub fn new(name: impl ToString) -> Self {
        Self {
            inner: Entity::new(name),
            lks: HashMap::new(),
        }
    }

    pub fn entity(&self) -> &Entity {
        &self.inner
    }

    pub fn get_lk(&self, name: impl ToString) -> StagResult<&(String, u8, u8)> {
        let str_name = name.to_string();
        match self.lks.get(&str_name) {
            Some(val) => Ok(val),
            _ => Err(StagError::NonexistantLink(str_name)),
        }
    }

    pub fn link_to(&mut self, entity: Entity) -> StagResult<()> {
        let str_name = entity.name();
        if self.lks.contains_key(&str_name) {
            Err(StagError::UnauthorizedLinkOverride)
        } else {
            self.lks.insert(str_name, (String::new(), 0, 0));
            Ok(())
        }
    }

    pub fn unlink(&mut self, entity: Entity) -> StagResult<()> {
        let str_name = entity.name();
        if self.lks.contains_key(&str_name) {
            self.lks.remove(&str_name);
            Ok(())
        } else {
            Err(StagError::NonexistantLink(str_name))
        }
    }
}
