mod mysql;

use crate::{constraint::SQLConstraint, entity::Entity};

#[derive(Clone)]
pub struct UniqueConstraint {
    name: String,
    entity: Entity,
    attrs: Vec<String>,
}

impl UniqueConstraint {
    pub fn new(name: impl ToString, entity: Entity) -> Self {
        Self {
            name: name.to_string(),
            entity,
            attrs: Vec::new(),
        }
    }

    pub fn attrs(&self) -> Vec<String> {
        self.attrs.clone()
    }
}

impl SQLConstraint for UniqueConstraint {
    fn entity(&self) -> Entity {
        self.entity.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
