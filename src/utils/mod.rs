mod action;
mod dialog;
mod files;

pub trait MDotActable {
    type InnerCallerType;
}

// re-exports
pub(crate) use action::*;
pub(crate) use dialog::*;
pub(crate) use files::*;
