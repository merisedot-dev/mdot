mod action;
mod dialog;

pub trait MDotActable {
    type InnerCallerType;
}

// re-exports
pub(crate) use action::*;
pub(crate) use dialog::*;
