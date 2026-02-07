use crate::{constants::DEFAULT_SCREEN_NAME, utils::MDotAction, window::imp::MDotWindow};

pub struct CloseAction;

impl MDotAction for CloseAction {
    type InnerCallerType = MDotWindow;

    fn name(&self) -> &'static str {
        "win.close"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        caller.page_stack.set_visible_child_name(DEFAULT_SCREEN_NAME);
        // TODO tweak project
    }
}
