mod builder;
mod overlay;

use crate::{
    entity::{Entity, EntityAttr, GraphLink},
    errors::StagResult,
};

/// Main logic trait of our graph handling, it will take an MCD or MLD [Graph] to
/// output a SQL script. Specifics will depend of the target SGBD, aka the
/// trait implementor. In case of errors in the parsed structs, the error will be
/// passed to the resulting script, and it's on the user to fix it.
pub trait ConversionCore {
    /// Checks if the given [EntityAttr] is of a valid type for the current
    /// [ConversionCore]. In case of it being valid, just return the [EntityAttr]
    /// back to caller (it's valid, up to the caller to do something with it).
    fn check_type(&self, attr: EntityAttr) -> StagResult<EntityAttr>;

    /// Gets an SQL-compliant header for the built script.
    fn header(&self, name: String) -> String;

    /// Turns a given [Entity] into an SQL-compliant script snippet. In case of
    /// anything going wrong, throws a [crate::errors::StagError::ParseError].
    fn entity(&self, entity: Entity) -> StagResult<String>;

    /// Turns a given [GraphLink] into an SQL-compliant script snippet. In case of
    /// anything going wrong, throws a [crate::errors::StagError::ParseError].
    fn link(&self, link: GraphLink) -> StagResult<String>;
}

// re-exports
pub use builder::*;
