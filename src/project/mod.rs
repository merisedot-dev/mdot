mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::{self, Object};

glib::wrapper! {pub struct Project(ObjectSubclass<imp::MDotProject>);}

impl Default for Project {
    fn default() -> Self {
        Object::builder()
            .property("name", String::new())
            .property("path", String::new())
            .build()
    }
}

// logic-based implementation of a project
impl Project {
    pub fn is_valid(&self) -> bool {
        let proj = self.imp().data.borrow();
        proj.name != "" && proj.path != "" // TODO other checks
    }
}
