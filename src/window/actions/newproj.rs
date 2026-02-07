use gtk::glib::property::PropertySet;

use crate::{
    constants::NEWPROJ_SCREEN_NAME, project::Project, utils::MDotAction, window::imp::MDotWindow,
};

/// win.mkproj GAction fixing struct. As for it and the others, it will just be
/// used to fix the operation, no associated value required.
pub struct MkProjAction;

impl MDotAction for MkProjAction {
    type InnerCallerType = MDotWindow;

    fn name(&self) -> &'static str {
        "win.mkproj"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        caller
            .page_stack
            .set_visible_child_name(NEWPROJ_SCREEN_NAME);
        caller.project.set(Project::default());
    }
}
