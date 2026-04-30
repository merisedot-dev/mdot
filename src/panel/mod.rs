mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;

use crate::constants::{DEFATUL_GRPHLK_NAME, DEFAULT_PANEL_NAME};

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
            .set_visible_child_name(DEFATUL_GRPHLK_NAME);
    }
}
