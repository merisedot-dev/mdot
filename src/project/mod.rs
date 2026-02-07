mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {pub struct Project(ObjectSubclass<imp::MDotProject>);}

impl Default for Project {
    fn default() -> Self {
        Object::builder().build()
    }
}

// logic-based implementation of a project
impl Project {
    pub fn is_valid(&self) -> bool {
        todo!("define what a valid project is")
    }
}
