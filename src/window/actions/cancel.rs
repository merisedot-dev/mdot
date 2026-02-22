use crate::{
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
        // erase information from form widgets
        caller.clear_form();
    }
}
