mod convert;
mod imp;

/// Generic SQL constraint encapsulation trait.
pub trait SQLConstraint {
    fn name(&self) -> String;
}

// re-exports
pub use imp::*;
