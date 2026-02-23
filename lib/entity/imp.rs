use std::collections::HashMap;

use crate::{
    entity::EntityAttr,
    errors::{StagError, StagResult},
};

/// Base entity definition. It will be used both for standalone entities and for
/// various GraphLinks in MCD and MLD graphs.
#[derive(Clone)]
pub struct Entity {
    name: String,
    attrs: HashMap<String, EntityAttr>,
}

impl Entity {
    /// Constructs a new [Entity], devoid of data.
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_attr(&self, name: impl ToString) -> StagResult<&EntityAttr> {
        let str_name = name.to_string();
        match self.attrs.get(&str_name) {
            Some(val) => Ok(val),
            None => Err(StagError::EntityAttributeNotFound(str_name)),
        }
    }
}
