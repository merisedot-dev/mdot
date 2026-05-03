use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::property::PropertySet;

use crate::{
    constants::{APP_NAME, NEWPROJ_SCREEN_NAME},
    utils::{MDotActable, MDotAction},
    window::{MDotWindow, project::Project},
};

/// win.mkproj GAction fixing struct. As for it and the others, it will just be
/// used to fix the operation, no associated value required.
pub struct MkProjAction;

impl MDotActable for MkProjAction {
    type InnerCallerType = MDotWindow;
}

impl MDotAction for MkProjAction {
    fn name(&self) -> &'static str {
        "win.mkproj"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        // UI tweaks
        caller.clear_form();
        caller.set_screen(NEWPROJ_SCREEN_NAME);
        caller.set_app_title(APP_NAME);
        caller.set_app_subtitle("");

        // replacing project
        caller.imp().project.set(Project::default());
    }
}
