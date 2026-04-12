use crate::{
    constraint::{ESQLConstraint, foreign::FKConstraint},
    entity::AttrRole,
    errors::{StagError, StagResult},
    graph::Graph,
    script::overlay::keys::Association,
};

/// Overlay built to capture [Constraint] information from any database graph.
/// This is not meant to add specific constraints, but to capture the more
/// common ones.
#[derive(Clone)]
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
                // the x,1~0,n situation type
                Association::ONE2MANY(name, nlb) => {
                    let ent = self.graph.edt_ent(&name)?;
                    // prefetches
                    let o_name = lk.other(&name)?;
                    let o_ent = temp_graph.get_ent(&o_name)?;
                    let (attr, _, _) = o_ent.get_attr(o_ent.get_pk()?)?;
                    // add missing key
                    ent.add_attr(&o_name, attr.clone(), AttrRole::FK, Some(nlb))?;
                    self.constraints
                        .push(ESQLConstraint::ForeignKey(FKConstraint::new(
                            format!("fk_lk_{}_{}", name, o_name),
                            o_name,
                            ent.clone(),
                            o_ent.clone(),
                        )?));
                    self.graph.del_lk(lk_name)?;
                }
                // one2one situations
                Association::ONE2ONE(nl_a, nl_b) => {
                    if let (false, false) = (nl_a, nl_b) {
                        // still need to have one nullable
                        return Err(StagError::ParseError);
                    } else {
                        // regular situation
                        let n1 = lk.get_lks().keys()[0].clone();
                        let n2 = lk.get_lks().keys()[1].clone();
                        // add missing keys (first entity)
                        self.graph.edt_ent(&n1)?.add_attr(
                            &n2,
                            temp_graph
                                .get_ent(&n2)?
                                .get_attr(temp_graph.get_ent(&n2)?.get_pk()?)?
                                .0
                                .clone(),
                            AttrRole::FK,
                            Some(nl_a),
                        )?;
                        // add missing keys (second entity)
                        self.graph.edt_ent(&n2)?.add_attr(
                            &n1,
                            temp_graph
                                .get_ent(&n1)?
                                .get_attr(temp_graph.get_ent(&n1)?.get_pk()?)?
                                .0
                                .clone(),
                            AttrRole::FK,
                            Some(nl_b),
                        )?;
                        // constraint on entity A
                        self.constraints
                            .push(ESQLConstraint::ForeignKey(FKConstraint::new(
                                format!("fk_{}_{}", n1, n2),
                                n2.clone(),
                                temp_graph.get_ent(&n1)?.clone(),
                                temp_graph.get_ent(&n2)?.clone(),
                            )?));
                        // constraint on entity B
                        self.constraints
                            .push(ESQLConstraint::ForeignKey(FKConstraint::new(
                                format!("fk_{}_{}", n2, n1),
                                n1.clone(),
                                temp_graph.get_ent(&n2)?.clone(),
                                temp_graph.get_ent(&n1)?.clone(),
                            )?));
                    }
                }
                // more than one on a side
                Association::MANY2MANY(name) | Association::TERNARY(name) => {
                    self.graph.mk_entity(&name)?;
                    let ent = self.graph.edt_ent(&name)?;
                    // add all required foreign keys
                    for (o_name, _) in lk.get_lks() {
                        let o_ent = temp_graph.get_ent(&o_name)?;
                        let o_attrs = o_ent.get_attr(o_ent.get_pk()?)?;
                        ent.add_attr(
                            &o_name,
                            o_attrs.clone().0,
                            AttrRole::FK, // forced foreign key
                            Some(false),  // no nullables here
                        )?;
                        // add constraints
                        self.constraints
                            .push(ESQLConstraint::ForeignKey(FKConstraint::new(
                                format!("fk_{}_{}", name, o_name),
                                o_name.clone(),
                                ent.clone(),
                                o_ent.clone(),
                            )?));
                    }
                    // add extra attributes
                    for (name, (attr, rl, nlb)) in lk.inner.get_all_attrs() {
                        ent.add_attr(name, attr.clone(), rl.clone(), Some(nlb))?;
                    }
                    // TODO check if a primary key is needed
                }
                _ => return Err(StagError::ParseError),
            }
        }
        Ok(())
    }
}
