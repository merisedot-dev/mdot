use crate::{
    constants::NEWPROJ_SCREEN_NAME,
    utils::{MDotActable, MDotAction},
    window::MDotWindow,
};

pub struct CloseAction;

impl MDotActable for CloseAction {
    type InnerCallerType = MDotWindow;
}

impl MDotAction for CloseAction {
    fn name(&self) -> &'static str {
        "win.close"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        caller.set_screen(NEWPROJ_SCREEN_NAME);
        // TODO tweak project
    }
}
