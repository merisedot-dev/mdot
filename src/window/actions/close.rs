use adw::subclass::prelude::ObjectSubclassIsExt;

use crate::{
    constants::DEFAULT_SCREEN_NAME,
    utils::{MDotActable, MDotAction},
    window::Window,
};

pub struct CloseAction;

impl MDotActable for CloseAction {
    type InnerCallerType = Window;
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
        caller
            .imp()
            .page_stack
            .set_visible_child_name(DEFAULT_SCREEN_NAME);
        // TODO tweak project
    }
}
