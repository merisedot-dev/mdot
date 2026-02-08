use adw::subclass::prelude::ObjectSubclassIsExt;

use crate::{constants::WORKS_SCREEN_NAME, utils::MDotAction, window::Window};

pub struct ValidateAction;

impl MDotAction for ValidateAction {
    type InnerCallerType = Window;

    fn name(&self) -> &'static str {
        "win.validate"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        let proj = caller.imp().project.borrow_mut();
        // TODO ensure all data is loaded
        // check if project is valid
        if proj.is_valid() {
            caller
                .imp()
                .page_stack
                .set_visible_child_name(WORKS_SCREEN_NAME);
            // TODO check for additional actions to fire
        }
    }
}
