use crate::{constraint::SQLConstraint, graph::Graph};

/// Overlay built to capture [Constraint] information from any database graph.
/// This is not meant to add specific constraints, but to capture the more
/// common ones.
pub struct GraphOverlay {
    graph: Graph,
    constraints: Vec<Box<dyn SQLConstraint + Send + Sync>>,
}

impl GraphOverlay {
    pub fn new(graph: Graph) -> Self {
        Self {
            graph,
            constraints: Vec::new(),
        }
    }
}
