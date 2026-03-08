use crate::{constraint::ESQLConstraint, errors::StagResult, graph::Graph};

/// Overlay built to capture [Constraint] information from any database graph.
/// This is not meant to add specific constraints, but to capture the more
/// common ones.
pub struct GraphOverlay {
    graph: Graph,
    constraints: Vec<ESQLConstraint>,
}

impl GraphOverlay {
    pub fn new(graph: Graph) -> Self {
        Self {
            graph,
            constraints: Vec::new(),
        }
    }

    pub fn graph(&self) -> Graph {
        self.graph.clone()
    }

    pub fn constraints(&self) -> Vec<ESQLConstraint> {
        self.constraints.clone()
    }

    /// Reduces graph to a set of a minimal [Graph] and a set of [ESQLConstraint].
    /// This should be called prior to any conversion into an SQL script.
    pub fn check(&mut self) -> StagResult<()> {
        let temp_graph = self.graph.clone(); // graph snapshot
        for (_, lk) in temp_graph.get_lks() {
            let lk_size = lk.get_all_lks().len();
            // checking situation
            if lk_size >= 3 {
                // TODO
            } else if lk_size == 2 {
                // TODO
            }
        }
        Ok(())
    }
}
