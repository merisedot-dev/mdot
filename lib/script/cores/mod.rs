mod mysql;

#[derive(Clone, Default, Eq, PartialEq)]
pub enum ExposedCore {
    MySQL(MySQLCore),
    #[default]
    NONE,
}

impl ExposedCore {
    pub fn name(&self) -> &str {
        match self {
            Self::MySQL(_) => "MySQL",
            Self::NONE => "",
        }
    }
}

impl From<String> for ExposedCore {
    fn from(value: String) -> Self {
        match value.as_str() {
            "MySQL" => Self::MySQL(MySQLCore),
            _ => Self::NONE,
        }
    }
}

// re-exports
pub use mysql::*;
