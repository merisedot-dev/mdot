use std::path::PathBuf;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::EditableExt;
use stag::script::ExposedCore;

use crate::{
    project::error::ProjectError,
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
        // load data from project (and drop borrow after for safety)
        {
            let proj = caller.imp().project.borrow_mut();
            // fetch from components
            proj.set_name(caller.imp().proj_name.text());
            proj.set_path(PathBuf::from(caller.imp().path_lbl.text()));
            proj.imp()
                .data
                .borrow_mut()
                .core
                .replace(ExposedCore::from(caller.get_selected_core()));
        }

        // fetch actual project
        let proj = caller.imp().project.borrow();
        // check project integrity
        if let Err(why) = proj.is_valid() {
            caller.show_form_err(format!("{}", why));
        }
    }
}
