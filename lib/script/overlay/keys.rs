use crate::{
    entity::{Cardi, GraphLink},
    errors::StagError,
};

/// Utility enumeration made to encapsulate how Merise associations work (and
/// therefore be translated to SQL links).
#[derive(Debug, Clone)]
pub enum Association {
    // Two-sided associations
    ONE2ONE(String),
    ONE2MANY(String),
    MANY2MANY,
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
                ((_, _, Cardi::MANY), (_, _, Cardi::MANY)) => Ok(Self::MANY2MANY),
                // one2many situations
                ((name_entity1, _, Cardi::ANY(_)), (_, _, Cardi::MANY)) => {
                    Ok(Self::ONE2MANY(name_entity1))
                }
                ((_, _, Cardi::MANY), (name_entity2, _, Cardi::ANY(_))) => {
                    Ok(Self::ONE2MANY(name_entity2))
                }
                // one2one situations
                ((_, _, Cardi::ANY(_)), (_, _, Cardi::ANY(_))) => todo!(),
                // aberrations situations
                _ => Err(StagError::ParseError),
            }
        } else {
            Err(StagError::ParseError)
        }
    }
}
