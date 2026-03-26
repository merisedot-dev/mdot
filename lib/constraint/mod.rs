mod convert;
mod imp;

use crate::entity::Entity;

/// Generic SQL constraint encapsulation trait. This is meant as both code
/// factorization and checks for any SQL converter.
pub trait SQLConstraint {
    /// Naming discriminator for each constraint. In case of multiple constraints
    /// having the same name, this may cause issues later.
    fn name(&self) -> String;

    /// The entity the constraint applies to.
    fn entity(&self) -> Entity;
}

// re-exports
pub use convert::*;
pub use imp::*;
