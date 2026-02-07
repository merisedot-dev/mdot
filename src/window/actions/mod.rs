mod close;
mod newproj;
mod validate;

use crate::{utils::MDotAction, window::imp::MDotWindow};

pub fn mk_actions() -> Vec<Box<dyn MDotAction<InnerCallerType = MDotWindow>>> {
    vec![
        Box::new(close::CloseAction),
        Box::new(newproj::MkProjAction),
        Box::new(validate::ValidateAction),
    ]
}
