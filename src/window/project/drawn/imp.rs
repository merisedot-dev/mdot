use std::cell::OnceCell;

use gtk::glib::{self, Properties, prelude::*, subclass::prelude::*};

#[derive(Default, Properties)]
#[properties(wrapper_type=super::DrawnElement)]
pub struct DrawnElement {
    #[property(get, set)]
    pub entity: OnceCell<String>,
    // TODO add coordinates
}

#[glib::object_subclass]
impl ObjectSubclass for DrawnElement {
    const NAME: &'static str = "DrawnElement";
    type Type = super::DrawnElement;
}

#[glib::derived_properties]
impl ObjectImpl for DrawnElement {}
