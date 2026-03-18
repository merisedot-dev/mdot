use crate::{
    entity::{Cardi, GraphLink},
    errors::StagError,
};

/// Utility enumeration made to encapsulate how Merise associations work (and
/// therefore be translated to SQL links). When considering booleans in there,
/// `true` only means it's nullable.
#[derive(Debug, Clone)]
pub enum Association {
    // Two-sided associations
    ONE2ONE(bool, bool),
    ONE2MANY(String, bool),
    MANY2MANY(bool, bool),
    // TODO ternaries (and more entities)
    // utility
    NONE,
}

impl Default for Association {
    fn default() -> Self {
        Self::NONE
    }
}

impl Association {
    fn concat(lk: GraphLink) -> Vec<(String, Cardi, Cardi)> {
        lk.get_lks()
            .iter()
            .map(|(k, (_, n, m))| (k.clone(), n.clone(), m.clone()))
            .collect()
    }
}

impl TryFrom<GraphLink> for Association {
    type Error = StagError;

    fn try_from(value: GraphLink) -> Result<Self, Self::Error> {
        let v_lks = Self::concat(value); // prefetch
        if v_lks.len() == 2 {
            // define cards value
            match (v_lks[0].clone(), v_lks[1].clone()) {
                // many2many situations
                ((_, m, Cardi::MANY), (_, n, Cardi::MANY)) => Ok(Self::MANY2MANY(
                    match m {
                        Cardi::ANY(_) => false,
                        _ => false,
                    },
                    match n {
                        Cardi::ANY(_) => false,
                        _ => false,
                    },
                )),
                // one2many situations
                ((ent_name, cardinality, Cardi::ANY(_)), (_, _, Cardi::MANY)) => {
                    Ok(Self::ONE2MANY(
                        ent_name,
                        match cardinality {
                            Cardi::ANY(_) => false,
                            _ => true,
                        },
                    ))
                }
                ((_, _, Cardi::MANY), (ent_name, cardinality, Cardi::ANY(_))) => {
                    Ok(Self::ONE2MANY(
                        ent_name,
                        match cardinality {
                            Cardi::ANY(_) => false,
                            _ => true,
                        },
                    ))
                }
                // one2one situations
                ((_, m, Cardi::ANY(_)), (_, n, Cardi::ANY(_))) => match (m, n) {
                    (Cardi::ZERO, Cardi::ANY(_)) => Ok(Self::ONE2ONE(true, false)),
                    (Cardi::ANY(_), Cardi::ZERO) => Ok(Self::ONE2ONE(false, true)),
                    (Cardi::ZERO, Cardi::ZERO) => Ok(Self::ONE2ONE(true, true)),
                    (_, _) => Err(StagError::ParseError),
                },
                // aberrations situations
                _ => Err(StagError::ParseError),
            }
        } else {
            todo!()
        }
    }
}
