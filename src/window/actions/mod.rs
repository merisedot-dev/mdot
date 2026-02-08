mod close;
mod newproj;
mod pick_proj;
mod validate;

use crate::{utils::MDotAction, window::Window};

pub fn mk_actions() -> Vec<Box<dyn MDotAction<InnerCallerType = Window>>> {
    vec![
        Box::new(close::CloseAction),
        Box::new(newproj::MkProjAction),
        Box::new(pick_proj::PickProjectAction),
        Box::new(validate::ValidateAction),
    ]
}
