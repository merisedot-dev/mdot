mod convert;
mod imp;

use crate::entity::Entity;

/// Generic SQL constraint encapsulation trait. This is meant as both code
/// factorization and checks for any SQL converter.
pub trait SQLConstraint {
    fn name(&self) -> String;
    fn entity(&self) -> Entity;
    fn target_attr(&self) -> String;
}

// re-exports
pub use convert::*;
pub use imp::*;
