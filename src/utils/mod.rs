mod action;
mod files;

pub trait MDotActable {
    type InnerCallerType;
}

// re-exports
pub(crate) use action::*;
