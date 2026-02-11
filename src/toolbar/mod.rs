use gtk::glib::{self, Object};

mod imp;

glib::wrapper! {
    pub struct Toolbar(ObjectSubclass<imp::MDotToolbar>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Toolbar {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
