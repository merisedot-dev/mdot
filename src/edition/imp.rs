use adw::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass};
use gtk::{Box, CompositeTemplate, glib, subclass::prelude::*};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/editionpanel.ui")]
pub struct MDotEditionPanel {}

// subclass core trait
#[glib::object_subclass]
impl ObjectSubclass for MDotEditionPanel {
    const NAME: &'static str = "MDotEditionPanel";
    type Type = super::EditionPanel;
    type ParentType = Box;
}

// main GObject trait
impl ObjectImpl for MDotEditionPanel {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

// core widget trait
impl WidgetImpl for MDotEditionPanel {}

// Box subclassing
impl BoxImpl for MDotEditionPanel {}
