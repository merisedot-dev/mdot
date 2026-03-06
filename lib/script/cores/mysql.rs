use crate::{
    constants::{MK_DB, MK_ENTITY},
    constraint::ESQLConstraint,
    entity::{Entity, EntityAttr, GraphLink},
    errors::{StagError, StagResult},
    script::ConversionCore,
};

/// MySQL-specific conversion core, it outputs elements of a correct MySQL script.
/// **Note**: It will not bother with any encoding nonsense, just clean SQL.
pub struct MySQLCore;

impl ConversionCore for MySQLCore {
    fn check_type(&self, attr: EntityAttr) -> StagResult<EntityAttr> {
        Ok(attr) // every attribute type is supported here
    }

    fn header(&self, name: String) -> String {
        format!("{} {};", MK_DB, name)
    }

    fn link(&self, link: GraphLink) -> StagResult<String> {
        self.entity(link.inner)
    }

    fn entity(&self, entity: Entity) -> StagResult<String> {
        Ok(format!("{} {} (\n\n);", MK_ENTITY, entity.name()))
    }

    fn constraint(&self, cstr: ESQLConstraint) -> StagResult<String> {
        match cstr {
            // TODO fetch converter for supported constraints
            _ => Err(StagError::ConstraintNotSupported),
        }
    }
}
