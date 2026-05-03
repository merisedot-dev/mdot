mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct MDotAttrRecord(ObjectSubclass<imp::MDotAttrRecord>);
}
