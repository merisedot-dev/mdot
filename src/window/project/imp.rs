use std::{cell::RefCell, path::PathBuf};

use gtk::{
    glib::{self, BoxedAnyObject, Properties},
    prelude::*,
    subclass::prelude::*,
};
use stag::{graph::Graph, script::ExposedCore};

/// Inner project data for MeriseDot. It is more of a placeholder struct, as the
/// true data will be held by the inner library (mostly the [Graph] struct).
#[derive(Clone, Default)]
pub struct ProjectData {
    pub name: String,  // project name, will not move at any cost
    pub path: PathBuf, // directory root path for the project
}

/// GTK implementor, it will be a storefront and value logic checker for the
/// [ProjectData] struct.
#[derive(Properties)]
#[properties(wrapper_type=super::Project)]
pub struct MDotProject {
    #[property(name="name", get, set, type = String, member = name)]
    #[property(name="path", get, set, type = PathBuf, member = path)]
    pub data: RefCell<ProjectData>,
    pub graph: BoxedAnyObject, // inner database graph (MCD by default)
    pub core: BoxedAnyObject,  // conversion core for the script
}

// subclassing trait
#[glib::object_subclass]
impl ObjectSubclass for MDotProject {
    const NAME: &'static str = "MDotProject";
    type Type = super::Project;
}

// necessary trait to subclass anything
#[glib::derived_properties]
impl ObjectImpl for MDotProject {}

/// manual default value in order to use [BoxedAnyObject].
impl Default for MDotProject {
    fn default() -> Self {
        Self {
            data: ProjectData::default().into(),
            graph: BoxedAnyObject::new(Graph::default()),
            core: BoxedAnyObject::new(ExposedCore::default()),
        }
    }
}
