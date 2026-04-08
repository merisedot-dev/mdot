use crate::{
    constants::{EDT_ENTITY, MK_CSTR},
    constraint::{
        SQLConstraint,
        convert::{SQLConverter, foreign::FKConstraint},
    },
    errors::StagResult,
    script::MySQLCore,
};

impl SQLConverter<FKConstraint> for MySQLCore {
    fn convert(&self, item: FKConstraint) -> StagResult<String> {
        Ok(format!(
            "{} {} {} {}\n\tforeign key({})\n\treferences {}.{};",
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
