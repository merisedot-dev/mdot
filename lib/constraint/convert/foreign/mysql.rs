use crate::{
    constants::{EDT_ENTITY, MK_CSTR},
    constraint::{
        SQLConstraint,
        convert::{SQLConverter, foreign::ForeignKeyConstraint},
    },
    errors::StagResult,
    script::MySQLCore,
};

impl SQLConverter<ForeignKeyConstraint> for MySQLCore {
    fn convert(&self, item: ForeignKeyConstraint) -> StagResult<String> {
        Ok(format!(
            "{} {}\n\t{} {}\n\tforeign key({}) references {}.{};",
            EDT_ENTITY,
            item.entity().name(),
            MK_CSTR,
            item.name(),
            item.target_attr(),
            item.reference().name(),
            item.reference().get_pk()?
        ))
    }
}
