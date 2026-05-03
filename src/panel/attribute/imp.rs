use gtk::{glib, subclass::prelude::*};

#[derive(Default)]
pub struct MDotAttrRecord {}

#[glib::object_subclass]
impl ObjectSubclass for MDotAttrRecord {
    const NAME: &'static str = "MDotAttrRecord";
    type Type = super::MDotAttrRecord;
}

// overriding core GObject trait
impl ObjectImpl for MDotAttrRecord {}
