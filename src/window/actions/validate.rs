use std::path::PathBuf;

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
        // ensure all data is loaded
        proj.set_name(caller.imp().proj_name.text());
        proj.set_path(PathBuf::from(caller.imp().path_lbl.label().to_string()));

        // check if project is valid
        if proj.is_valid() {
            // widget tweaks
            caller.set_app_title(proj.get_name());
            caller.set_app_subtitle(proj.filepath().to_str().unwrap_or_default());
            // change screen
            caller.set_screen(WORKS_SCREEN_NAME);
        }
    }
}
