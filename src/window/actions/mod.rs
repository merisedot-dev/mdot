mod core;
mod entity;

use crate::{utils::MDotAction, window::MDotWindow};

pub fn mk_actions() -> Vec<Box<dyn MDotAction<InnerCallerType = MDotWindow>>> {
    vec![
        Box::new(core::cancel::CancelAction),
        Box::new(core::close::CloseAction),
        Box::new(core::newproj::MkProjAction),
        Box::new(core::validate::ValidateAction),
        Box::new(core::save::SaveAction),
        // entity-related stuff
        Box::new(entity::new::NewEntityAction),
    ]
}
