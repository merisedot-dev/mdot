use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::EditableExt;

use crate::{
    constants::WORKS_SCREEN_NAME,
    utils::{MDotActable, MDotAction},
    window::Window,
};

pub struct ValidateAction;

impl MDotActable for ValidateAction {
    type InnerCallerType = Window;
}

impl MDotAction for ValidateAction {
    fn name(&self) -> &'static str {
        "win.validate"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        let proj = caller.imp().project.borrow();
        // TODO ensure all data is loaded
        proj.set_name(caller.imp().proj_name.text());
        // check if project is valid
        if proj.is_valid() {
            caller.set_screen(WORKS_SCREEN_NAME);
            // ensure project file exists
            if !proj.filepath().exists() {
                // TODO create new file
                // nothing to write in it for now
            }
        }
    }
}
