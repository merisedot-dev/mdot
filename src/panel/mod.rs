mod attribute;
mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;

use crate::constants::{
    DEFAULT_GRPHLK_NAME, DEFAULT_PANEL_NAME, EDIT_PANEL_PAGE_NAME, LINKS_STACK_EDIT_PAGE_NAME,
};

glib::wrapper! {
    pub struct MDotPanel(ObjectSubclass<imp::MDotPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Root;
}

// creation implementation
impl MDotPanel {
    /// Loads default values into the build [MDotPanel]. This is meant to be
    /// called only at construction.
    fn set_defaults(&self) {
        self.imp()
            .panel_stack
            .set_visible_child_name(DEFAULT_PANEL_NAME);
        self.imp()
            .links_stack
            .set_visible_child_name(DEFAULT_GRPHLK_NAME);
    }

    /// Changes the stack page of the [MDotPanel] for edition. This will also
    /// tweak some substacks depending if they're needed.
    pub fn start_edit(&self, has_links: bool) {
        // core page
        self.imp()
            .panel_stack
            .set_visible_child_name(EDIT_PANEL_PAGE_NAME);
        // substacks
        if has_links {
            self.imp()
                .links_stack
                .set_visible_child_name(LINKS_STACK_EDIT_PAGE_NAME);
        }
    }
}
