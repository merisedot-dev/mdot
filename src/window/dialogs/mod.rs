use crate::{utils::MDotActable, window::Window};

pub struct PickProjDialog;

impl MDotActable for PickProjDialog {
    type InnerCallerType = Window;
}
