mod imp;

use std::path::PathBuf;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct Project(ObjectSubclass<imp::MDotProject>);
}

impl Default for Project {
    fn default() -> Self {
        Object::builder()
            .property("name", String::new())
            .property("path", PathBuf::new())
            .build()
    }
}

// Utitility-based implementation of Project
impl Project {
    pub fn get_path(&self) -> PathBuf {
        self.imp().data.borrow().path.clone()
    }

    pub fn get_name(&self) -> String {
        self.imp().data.borrow().name.clone()
    }
}

// logic-based implementation of a project
impl Project {
    pub fn is_valid(&self) -> bool {
        self.get_name() != "" && self.get_path().to_str().unwrap_or_default() != "" // TODO other checks
    }

    pub fn filepath(&self) -> PathBuf {
        let mut path = self.get_path();
        path.push(self.get_name());
        path
    }
}
