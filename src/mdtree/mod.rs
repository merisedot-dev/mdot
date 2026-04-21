mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct MDotGraphTree(ObjectSubclass<imp::MDotGraphTree>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Root;
}
