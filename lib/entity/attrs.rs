/// Enumeration for every possible attribute type available to an SGBD.
/// Each conversion core should exclude the ones it does not want.
#[derive(Clone, Debug)]
pub enum EntityAttr {
    BOOLEAN,
    // numbers
    INTEGER,
}

impl ToString for EntityAttr {
    fn to_string(&self) -> String {
        format!(
            "{}",
            match self {
                Self::BOOLEAN => "bool",
                Self::INTEGER => "int",
            }
        )
    }
}
