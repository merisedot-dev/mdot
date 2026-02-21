mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct EditionPanel(ObjectSubclass<imp::MDotEditionPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for EditionPanel {
    fn default() -> Self {
        Object::builder().build()
    }
}
