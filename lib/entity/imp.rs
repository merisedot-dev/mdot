use std::collections::HashMap;

use crate::{
    entity::EntityAttr,
    errors::{StagError, StagResult},
};

/// Base entity definition. It will be used both for standalone entities and for
/// various GraphLinks in MCD and MLD graphs.
#[derive(Clone, Debug, Default)]
pub struct Entity {
    name: String,
    attrs: HashMap<String, EntityAttr>,
}

impl Entity {
    /// Constructs a new [Entity], devoid of data. Because the name will be used
    /// as ID, it will be lowercased.
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string().to_lowercase(),
            attrs: HashMap::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Fetches attribute value from [Entity].
    ///
    /// **Warning**: In case of missing attribute, throws a
    /// [StagError::EntityAttributeNotFound] error.
    pub fn get_attr(&self, name: impl ToString) -> StagResult<&EntityAttr> {
        let str_name = name.to_string().to_lowercase();
        match self.attrs.get(&str_name) {
            Some(val) => Ok(val),
            None => Err(StagError::EntityAttributeNotFound(str_name)),
        }
    }

    /// Add a new attribute to the current [Entity]. Since every name is going to
    /// act as ID, it will be lowercased.
    ///
    /// **Warning**: In case of already existing attributes, it will throw a
    /// [StagError::EntityWrongAttributeOverride] error.
    pub fn add_attr(
        &mut self,
        name: impl ToString,
        attribute_typevalue: EntityAttr,
    ) -> StagResult<()> {
        let str_name = name.to_string().to_lowercase();
        if self.attrs.contains_key(&str_name) {
            Err(StagError::EntityWrongAttributeOverride(str_name))
        } else {
            self.attrs.insert(str_name, attribute_typevalue);
            Ok(())
        }
    }
}
