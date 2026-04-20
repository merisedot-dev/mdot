use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct MDotToolbar(ObjectSubclass<imp::MDotToolbar>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget, gtk::Root;
}
