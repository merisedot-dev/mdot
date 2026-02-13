use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::property::PropertySet;

use crate::{
    constants::NEWPROJ_SCREEN_NAME,
    project::Project,
    utils::{MDotActable, MDotAction},
    window::Window,
};

/// win.mkproj GAction fixing struct. As for it and the others, it will just be
/// used to fix the operation, no associated value required.
pub struct MkProjAction;

impl MDotActable for MkProjAction {
    type InnerCallerType = Window;
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
        caller.set_screen(NEWPROJ_SCREEN_NAME);
        caller.imp().project.set(Project::default()); // replacing project with default
    }
}
