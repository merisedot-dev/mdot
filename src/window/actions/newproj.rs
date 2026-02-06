use gtk::glib::Variant;

use crate::{utils::MDotAction, window::imp::MDotWindow};

pub struct MkProjAction;

impl MDotAction for MkProjAction {
    const NAME: &'static str = "mkproj";

    type InnerCallerType = MDotWindow;

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        text: &str,
        variant: Option<&Variant>,
    ) {
        // TODO
    }
}
