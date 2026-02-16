use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum LocalizedProject {
    None,
}

impl Default for LocalizedProject {
    fn default() -> Self {
        Self::None
    }
}

impl Serialize for LocalizedProject {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for LocalizedProject {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
