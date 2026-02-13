mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct EditionPanel(ObjectSubclass<imp::MDotEditionPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
