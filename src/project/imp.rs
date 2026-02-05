use gtk::{glib, glib::Properties, subclass::prelude::*};

#[derive(Default, Properties)]
#[properties(wrapper_type=super::Project)]
pub struct MDotProject {}

// subclassing trait
#[glib::object_subclass]
impl ObjectSubclass for MDotProject {
    const NAME: &'static str = "MDotProject";
    type Type = super::Project;
}

// necessary trait to subclass anything
#[glib::derived_properties]
impl ObjectImpl for MDotProject {}
