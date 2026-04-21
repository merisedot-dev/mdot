pub(crate) mod error;
mod imp;

use std::{ops::Deref, path::PathBuf};

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::{self, Object};
use stag::script::ExposedCore;

use crate::{constants::PROJ_FILE_EXTENSION, project::error::ProjectError};

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
    /// Fetches the root path for the [Project]. This will not fetch the index
    /// file path of said [Project].
    pub fn get_dir_path(&self) -> PathBuf {
        self.imp().data.borrow().path.clone()
    }

    pub fn get_name(&self) -> String {
        self.imp().data.borrow().name.clone()
    }
}

// logic-based implementation of a project
impl Project {
    /// Checks if the current [Project] is in a usable state. This means making
    /// sure the [Project] has a valid name and its directory root path isn't
    /// an empty path. Individual indexes should be checked later on.
    pub fn is_valid(&self) -> Result<(), ProjectError> {
        let proj = self.imp().data.borrow();
        // simple project checks
        if proj.name == String::new() {
            return Err(ProjectError::MISSINGNAME);
        }
        if proj.path == PathBuf::new() {
            return Err(ProjectError::MISSINGPATH);
        }
        if proj.core.borrow().deref().clone() == ExposedCore::default() {
            return Err(ProjectError::MISSINGCORE);
        }
        // everything passed
        Ok(())
    }

    /// Fetches the index file path for the current [Project].
    pub fn filepath(&self) -> PathBuf {
        let mut path = self.get_dir_path();
        path.push(format!("{}{}", self.get_name(), PROJ_FILE_EXTENSION));
        path
    }
}
