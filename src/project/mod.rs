mod imp;

use std::{collections::HashMap, path::PathBuf};

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
        proj.name != "" // TODO other checks
    }

    pub fn get_file_paths(&self) -> HashMap<String, PathBuf> {
        self.imp().data.borrow().files.clone()
    }
}
