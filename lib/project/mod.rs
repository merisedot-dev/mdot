pub mod enumerate;

use std::collections::HashMap;

use crate::{attribute::EntityAttribute, entity::Entity};

/// Logic processing struct for a database design project. It will hold
/// all logic-based info, abstract from the target SGBD.
#[derive(Clone)]
pub struct StagProject<T>
where
    T: EntityAttribute,
{
    entities: HashMap<String, Entity<T>>,
}

impl<T> StagProject<T>
where
    T: EntityAttribute,
{
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }
}
