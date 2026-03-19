use crate::{
    entity::{Cardi, GraphLink},
    errors::StagError,
};

/// Utility enumeration made to encapsulate how Merise associations work (and
/// therefore be translated to SQL links). When considering booleans in there,
/// `true` only means it's nullable.
#[derive(Clone, Debug, Default)]
pub enum Association {
    // Two-sided associations
    ONE2ONE(bool, bool),
    ONE2MANY(String, bool),
    MANY2MANY(String),
    // TODO ternaries (and more entities)
    // utility
    #[default]
    NONE,
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
                ((name_1, _, Cardi::MANY), (name_2, _, Cardi::MANY)) => {
                    Ok(Self::MANY2MANY(format!("lk_{}_{}", name_1, name_2)))
                }
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
                (
                    (_, min_cardinality_ent1, Cardi::ANY(_)),
                    (_, min_cardinality_ent2, Cardi::ANY(_)),
                ) => Ok(Self::ONE2ONE(
                    match min_cardinality_ent1 {
                        Cardi::ZERO => true,
                        _ => false,
                    },
                    match min_cardinality_ent2 {
                        Cardi::ZERO => true,
                        _ => false,
                    },
                )),
                // aberrations situations
                _ => Err(StagError::ParseError),
            }
        } else {
            todo!()
        }
    }
}
