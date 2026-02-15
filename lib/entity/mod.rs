mod attr;

use std::collections::HashMap;

use crate::{
    attribute::EntityAttribute,
    entity::attr::AttrValue,
    error::{StagError, StagResult},
};

/// A single MCD/MLD/SQL entity/table, tuned to the target SGBD the user want
/// to design for. Please refer to the [EntityAttribute] documentation to make
/// sure your SGBD is supported.
#[derive(Clone)]
pub struct Entity<T>
where
    T: EntityAttribute,
{
    name: String,
    attrs: HashMap<String, AttrValue<T>>,
}

impl<T> Entity<T>
where
    T: EntityAttribute,
{
    /// Create a new [Entity] devoid of any attribute. The name shall not be
    /// changed as it will act as its identifier (so it will be lowercased).
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string().to_lowercase(),
            attrs: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_attr(&self, name: impl ToString) -> StagResult<&AttrValue<T>> {
        match self.attrs.get(&name.to_string()) {
            Some(val) => Ok(val),
            _ => Err(StagError::AttributeNotFound),
        }
    }

    pub fn add_attr(&mut self, name: impl ToString, nullable: bool, attr: T) {
        self.attrs
            .insert(name.to_string(), AttrValue::new(nullable, attr));
    }
}
