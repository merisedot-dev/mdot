mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {pub struct Project(ObjectSubclass<imp::MDotProject>);}

impl Default for Project{
    fn default() -> Self {
        Object::builder().build()
    }
}
