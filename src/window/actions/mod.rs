mod cancel;
mod close;
mod newproj;
mod validate;

use crate::{utils::MDotAction, window::Window};

pub fn mk_actions() -> Vec<Box<dyn MDotAction<InnerCallerType = Window>>> {
    vec![
        Box::new(cancel::CancelAction),
        Box::new(close::CloseAction),
        Box::new(newproj::MkProjAction),
        Box::new(validate::ValidateAction),
    ]
}
