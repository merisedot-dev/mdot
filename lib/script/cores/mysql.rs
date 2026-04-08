use crate::{
    constants::{MK_DB, MK_ENTITY},
    constraint::{ESQLConstraint, SQLConverter},
    entity::{AttrRole, Entity, EntityAttr, GraphLink},
    errors::StagResult,
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
        Ok(format!(
            "{} {} (\n\t{}\n);",
            MK_ENTITY,
            entity.name(),
            entity
                .get_all_attrs()
                .iter()
                .map(|(name, (atype, role, nlb))| format!(
                    "{} {}{}",
                    name,
                    atype,
                    match role {
                        AttrRole::PK => " primary key not null",
                        _ => match nlb {
                            false => " not null",
                            true => "", // nothing here
                        },
                    }
                ))
                .collect::<Vec<String>>()
                .join(",\n\t")
        ))
    }

    fn constraint(&self, cstr: ESQLConstraint) -> StagResult<String> {
        match cstr {
            ESQLConstraint::ForeignKey(cstr) => self.convert(cstr),
            ESQLConstraint::Unique(cstr) => self.convert(cstr),
        }
    }
}
