use crate::{
    constraint::{ESQLConstraint, foreign::FKConstraint},
    entity::AttrRole,
    errors::StagResult,
    graph::Graph,
    script::overlay::keys::Association,
};

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

    pub fn check(&mut self) -> StagResult<()> {
        self.to_mld()
    }

    /// Turns the inner [Graph] into an MLD graph. This will remove now useless
    /// GraphLink and edit any existing Entity to slot in foreign keys where
    /// needed (notably for one2one and one2many associations).
    ///
    /// **Warning**: In case of wrongful conversion to MLD, throws a
    /// [StagError::ParseError] back to caller.
    fn to_mld(&mut self) -> StagResult<()> {
        let temp_graph = self.graph.clone(); // graph snapshot
        for (lk_name, lk) in temp_graph.get_lks() {
            match Association::try_from(lk.clone())? {
                Association::ONE2MANY(name, nlb) => {
                    let ent = self.graph.edt_entity(&name)?;
                    // prefetches
                    let o_name = lk.other(&name)?;
                    let t_ent = temp_graph.get_ent(&name)?;
                    let (attr, _, _) = t_ent.get_attr(t_ent.get_pk()?)?.clone();
                    // add missing key
                    ent.add_attr(&o_name, attr.clone(), AttrRole::FK, Some(nlb))?;
                    self.constraints
                        .push(ESQLConstraint::ForeignKey(FKConstraint::new(
                            format!("lk_{}_{}", name, o_name),
                            o_name,
                            ent.clone(),
                            t_ent.clone(),
                        )?));
                    self.graph.del_lk(lk_name)?;
                }
                _ => todo!(),
            }
        }
        Ok(())
    }
}
