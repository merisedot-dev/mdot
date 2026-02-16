pub mod enumerate;

use crate::{attribute::EntityAttribute, graph::MeriseGraph};

/// Logic processing struct for a database design project. It will hold
/// all logic-based info, abstract from the target SGBD.
#[derive(Clone)]
pub struct StagProject<T>
where
    T: EntityAttribute,
{
    graph: MeriseGraph<T>,
}

impl<T> StagProject<T>
where
    T: EntityAttribute,
{
    pub fn new() -> Self {
        Self {
            graph: MeriseGraph::new(),
        }
    }
}
