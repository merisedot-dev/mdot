mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct MDotGraphRow(ObjectSubclass<imp::MDotGraphRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}
