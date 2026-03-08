use crate::{
    constants::{EDT_ENTITY, MK_CSTR},
    constraint::{SQLConstraint, SQLConverter, unique::UniqueConstraint},
    errors::StagResult,
    script::MySQLCore,
};

impl SQLConverter<UniqueConstraint> for MySQLCore {
    fn convert(&self, item: UniqueConstraint) -> StagResult<String> {
        Ok(format!(
            "{} {}\n\t{} {}\n\tunique ({});",
            EDT_ENTITY,
            item.entity().name(),
            MK_CSTR,
            item.name(),
            item.attrs()
                .iter()
                .map(|i| i.clone())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    }
}
