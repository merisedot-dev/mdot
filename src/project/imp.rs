use gtk::{glib, glib::Properties, subclass::prelude::*};

#[derive(Default, Properties)]
#[properties(wrapper_type=super::Project)]
pub struct MDotProject {
    pub name: String,
    // TODO define project index information
    // TODO define path map
    // TODO find a way to save files hash (just for safety)
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
