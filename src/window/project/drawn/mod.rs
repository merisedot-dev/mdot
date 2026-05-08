mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct DrawnElement(ObjectSubclass<imp::DrawnElement>);
}

impl DrawnElement {
    /// Builds a new [DrawnElement] from scratch.
    pub fn new(ent_name: impl ToString) -> Self {
        Object::builder()
            .property("entity", ent_name.to_string())
            .build()
    }
}
