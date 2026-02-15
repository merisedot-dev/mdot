use std::{cell::RefCell, path::PathBuf};

use gtk::{glib, glib::Properties, prelude::*, subclass::prelude::*};
use serde::{Deserialize, Serialize};
use stag::project::enumerate::LocalizedProject;

/// Inner project data for MeriseDot. It is more of a placeholder struct, as the
/// true data will be held by the inner library.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProjectData {
    pub name: String,
    pub path: PathBuf,
    pub stag_proj: LocalizedProject,
}

/// GTK implementor, it will be a storefront and value logic checker for the
/// [ProjectData] struct.
#[derive(Default, Properties)]
#[properties(wrapper_type=super::Project)]
pub struct MDotProject {
    #[property(name="name", get, set, type = String, member = name)]
    #[property(name="path", get, set, type = PathBuf, member = path)]
    pub data: RefCell<ProjectData>,
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
