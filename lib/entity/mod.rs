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

    /// Fetches [Entity] name, this will however be kept immutable.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Fetches every known attribute name in the current [Entity].
    /// The returned values may not be sorted.
    pub fn get_attrs_names(&self) -> impl Iterator {
        self.attrs.keys()
    }

    /// Fetch relevant attribute value depending on the given name. To be simpler,
    /// errors will be coerced into the [StagError] error enumeration.
    ///
    /// **Note** : Due to the name being used as ID, it will be lowercased.
    pub fn get_attr(&self, name: impl ToString) -> StagResult<&AttrValue<T>> {
        let str_name = name.to_string().to_lowercase();
        match self.attrs.get(&str_name) {
            Some(val) => Ok(val),
            None => Err(StagError::AttributeNotFound(str_name)),
        }
    }

    /// Add a new attribute in our [Entity]. Keep in mind the `name` parameter
    /// will be lowercased to avoid ID confusion.
    pub fn add_attr(&mut self, name: impl ToString, nullable: bool, attr: T) {
        self.attrs
            .insert(name.to_string(), AttrValue::new(nullable, attr));
    }

    /// Remove an existing attribute from the current [Entity]. In case of
    /// missing attribute, please check the [StagError] type for info.
    pub fn del_attr(&mut self, name: impl ToString) -> StagResult<()> {
        let str_name = name.to_string().to_lowercase();
        match self.attrs.remove(&str_name) {
            Some(_) => Ok(()),
            None => Err(StagError::AttributeNotFound(str_name)),
        }
    }
}
