mod newproj;
mod open;

use crate::{utils::MDotAction, window::imp::MDotWindow};

pub fn mk_actions() -> Vec<Box<dyn MDotAction<InnerCallerType = MDotWindow>>> {
    vec![Box::new(newproj::MkProjAction), Box::new(open::OpenAction)]
}
