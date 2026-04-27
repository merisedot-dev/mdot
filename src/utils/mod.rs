mod action;
mod config;
mod loaders;

/// Root trait for anything that can act upon a custom GTK component.
/// Specialized traits should require this to be implemented.
pub trait MDotActable {
    /// Refers to the type of the component that can be acted upon.
    type InnerCallerType;
}

// re-exports
pub(crate) use action::*;
pub(crate) use config::*;
pub(crate) use loaders::*;
