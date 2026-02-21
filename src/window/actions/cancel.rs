use crate::{
    constants::NEWPROJ_SCREEN_NAME,
    utils::{MDotActable, MDotAction},
    window::Window,
};

pub struct CancelAction;

impl MDotActable for CancelAction {
    type InnerCallerType = Window;
}

impl MDotAction for CancelAction {
    fn name(&self) -> &'static str {
        "win.cancel"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        // TODO erase information from form widgets
        caller.clear_form();
        // jump back to main page
        caller.set_screen(NEWPROJ_SCREEN_NAME);
    }
}
