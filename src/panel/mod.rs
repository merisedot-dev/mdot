mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct MDotPanel(ObjectSubclass<imp::MDotPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Root;
}
