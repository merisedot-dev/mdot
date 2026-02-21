use std::collections::HashMap;

use crate::entity::EntityAttribute;

/// Base entity definition. It will be used both for standalone entities and for
/// various [GraphLink]s in MCD and MLD graphs.
pub struct Entity<T>
where
    T: EntityAttribute,
{
    name: String,
    attrs: HashMap<String, T>,
}

impl<T> Entity<T>
where
    T: EntityAttribute,
{
    /// Constructs a new [Entity], devoid of data.
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }
}
