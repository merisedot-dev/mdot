use adw::subclass::prelude::ObjectSubclassIsExt;

use crate::{
    utils::{MDotActable, MDotAction},
    window::Window,
};

pub struct SaveAction;

impl MDotActable for SaveAction {
    type InnerCallerType = Window;
}

impl MDotAction for SaveAction {
    fn name(&self) -> &'static str {
        "win.save"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        // keeping proj reference for later
        let proj = caller.imp().project.borrow();

        // TODO open project file
        let path = proj.path();

        // open extra files
        for (_, path) in proj.get_file_paths() {
            // TODO write info
        }
    }
}
