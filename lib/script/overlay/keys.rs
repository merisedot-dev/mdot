use crate::entity::GraphLink;

/// Utility enumeration made to encapsulate how Merise associations work (and
/// therefore be translated to SQL links).
#[derive(Clone)]
pub enum Association {
    // Two-sided associations
    ONE2ONE(String),
    ONE2MANY(String),
    MANY2MANY,
    // TODO ternaries (and more entities)
    // utility
    NONE,
}

impl From<GraphLink> for Association {
    fn from(value: GraphLink) -> Self {
        let v_size = value.get_all_lks().len();
        if v_size == 2 {
            let v_lks = value
                .get_all_lks()
                .values()
                .into_iter()
                .map(|i| i.clone())
                .collect::<Vec<(String, u8, u8)>>();
            // TODO define cards value
            todo!()
        } else {
            Self::NONE // failsafe value
        }
    }
}
