use std::{cell::RefCell, collections::HashMap, path::PathBuf};

use gtk::{glib, glib::Properties, prelude::*, subclass::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProjectData {
    pub name: String,
    pub path: PathBuf,
    pub files: HashMap<String, PathBuf>,
}

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
