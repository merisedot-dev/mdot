// string constants
const PK_NAME: &'static str = "PK";
const FK_NAME: &'static str = "FK";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttrRole {
    PK,
    FK,
    None,
}

impl Default for AttrRole {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for AttrRole {
    fn to_string(&self) -> String {
        format!(
            "{}",
            match self {
                Self::PK => PK_NAME,
                Self::FK => FK_NAME,
                Self::None => "", // nothing here
            }
        )
    }
}

impl From<String> for AttrRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            PK_NAME => Self::PK,
            FK_NAME => Self::FK,
            _ => Self::None,
        }
    }
}
