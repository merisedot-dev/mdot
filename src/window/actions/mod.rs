mod cancel;
mod close;
mod newproj;
mod save;
mod validate;

use crate::{utils::MDotAction, window::MDotWindow};

pub fn mk_actions() -> Vec<Box<dyn MDotAction<InnerCallerType = MDotWindow>>> {
    vec![
        Box::new(cancel::CancelAction),
        Box::new(close::CloseAction),
        Box::new(newproj::MkProjAction),
        Box::new(validate::ValidateAction),
        Box::new(save::SaveAction),
    ]
}
