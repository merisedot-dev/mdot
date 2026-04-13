use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{AttrRole, EntityAttr},
    errors::{StagError, StagResult},
};

/// Base entity definition. It will be used both for standalone entities and for
/// various GraphLinks in MCD and MLD graphs.
#[derive(Clone, Debug, Default, Eq, Serialize, Deserialize)]
pub struct Entity {
    name: String,
    attrs: IndexMap<String, (EntityAttr, AttrRole, bool)>,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Entity {
    /// Constructs a new [Entity], devoid of data. Because the name will be used
    /// as ID, it will be lowercased.
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string().to_lowercase(),
            attrs: IndexMap::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn get_all_attrs(&self) -> IndexMap<String, (EntityAttr, AttrRole, bool)> {
        self.attrs.clone()
    }

    /// Fetches attribute value from [Entity], as well as its role (PK, FK...).
    ///
    /// **Warning**: In case of missing attribute, throws a
    /// [StagError::EntityAttributeNotFound] error.
    pub fn get_attr(
        &self,
        attribute_strname: impl ToString,
    ) -> StagResult<&(EntityAttr, AttrRole, bool)> {
        let str_name = attribute_strname.to_string().to_lowercase();
        match self.attrs.get(&str_name) {
            Some(val) => Ok(val),
            None => Err(StagError::EntityAttributeNotFound(str_name)),
        }
    }

    /// Fetches the primary key of the current [Entity]. In case of multiple
    /// primary keys, just returns the first one found.
    ///
    /// **Warning**: If there is no primary key in the entity, throws a
    /// [StagError::NoPK] error back to caller.
    pub fn get_pk(&self) -> StagResult<String> {
        match self
            .get_all_attrs()
            .iter()
            .find_map(|(name, (_, role, _))| {
                if role.clone() == AttrRole::PK {
                    Some(name)
                } else {
                    None
                }
            }) {
            Some(name) => Ok(name.clone()),
            None => Err(StagError::NoPK),
        }
    }

    /// Add a new attribute to the current [Entity]. Since every name is going to
    /// act as ID, it will be lowercased.
    ///
    /// **Warning**: In case of already existing attributes, it will throw a
    /// [StagError::EntityWrongAttributeOverride] error.
    pub fn add_attr(
        &mut self,
        name: impl ToString,
        attribute_typevalue: EntityAttr,
        role: AttrRole,
        nullable: Option<bool>,
    ) -> StagResult<()> {
        let str_name = name.to_string().to_lowercase();
        if self.attrs.contains_key(&str_name) {
            Err(StagError::EntityWrongAttributeOverride(str_name))
        } else {
            self.attrs.insert(
                str_name,
                (
                    attribute_typevalue,
                    role,
                    match nullable {
                        Some(val) => val,
                        None => true,
                    },
                ),
            );
            Ok(())
        }
    }

    /// Deletes an existing attribute from the current [Entity].
    ///
    /// **Warning**: In case of the attribute unable to be deleted, throws a
    /// [StagError::EntityAttributeNotFound] error.
    pub fn del_attr(&mut self, name: impl ToString) -> StagResult<()> {
        let str_name = name.to_string().to_lowercase();
        match self.attrs.shift_remove(&str_name) {
            Some(_) => Ok(()),
            None => Err(StagError::EntityAttributeNotFound(str_name)),
        }
    }
}
