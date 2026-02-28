use crate::{entity::EntityAttr, errors::StagResult};

/// Main logic trait of our graph handling, it will take an MCD or MLD [Graph] to
/// output a SQL script. Specifics will depend of the target SGBD, aka the
/// trait implementor. In case of errors in the [Graph], the error will be
/// passed to the resulting script, and it's on the user to fix it.
pub trait ConversionCore {
    /// Checks if the given [EntityAttr] is of a valid type for the current
    /// [ConversionCore]. In case of it being valid, just return the [EntityAttr]
    /// back to caller (it's valid, up to the caller to do something with it).
    fn check_type(&self, attr: EntityAttr) -> StagResult<EntityAttr>;
}

// re-exports
