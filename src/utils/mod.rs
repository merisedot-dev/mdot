mod action;
mod loaders;

pub trait MDotActable {
    type InnerCallerType;
}

// re-exports
pub(crate) use action::*;
pub(crate) use loaders::*;
