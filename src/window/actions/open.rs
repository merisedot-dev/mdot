use crate::{utils::MDotAction, window::imp::MDotWindow};

pub struct OpenAction;

impl MDotAction for OpenAction {
    type InnerCallerType = MDotWindow;

    fn name(&self) -> &'static str {
        "open"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        // TODO later
    }
}
