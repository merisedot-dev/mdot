pub(crate) mod error;
mod imp;

use std::{cell::Ref, ops::Deref, path::PathBuf};

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib::{self, Object};
use stag::{graph::Graph, script::ExposedCore};

use crate::{constants::PROJ_FILE_EXTENSION, window::project::error::ProjectError};

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

    /// Fetches the [Project]'s name. Meant to be used for display purposes.
    pub fn get_name(&self) -> String {
        self.imp().data.borrow().name.clone()
    }
}

// graph implementation
impl Project {
    /// Fetches the inner [Graph] of the given [Project]. It should allow for
    /// mutable methods to be called if required (please don't use them if not
    /// necessary).
    pub fn get_graph(&self) -> Ref<'_, Graph> {
        self.imp().graph.borrow()
    }

    /// Swaps the old stored [Graph] with the new one, the old value being dropped
    /// in the process.
    pub fn set_graph(&self, graph: Graph) {
        self.imp().graph.replace(graph);
    }
}

// conversion core implementation
impl Project {
    /// Fetches the [ExposedCore] for conversion purposes.
    pub fn get_core(&self) -> Ref<'_, ExposedCore> {
        self.imp().core.borrow()
    }

    /// Changes the crrent [ExposedCore] of the project.
    pub fn set_core(&self, core: ExposedCore) {
        self.imp().core.replace(core);
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
        if *self.get_core().deref() == ExposedCore::default() {
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
