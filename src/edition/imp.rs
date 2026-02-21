use adw::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass};
use gtk::{Box, CompositeTemplate, glib, subclass::prelude::*};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/editionpanel.ui")]
pub struct MDotEditionPanel {
    // TODO add important components
}

// subclass core trait
#[glib::object_subclass]
impl ObjectSubclass for MDotEditionPanel {
    const NAME: &'static str = "MDotEditionPanel";
    type Type = super::EditionPanel;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        // link the template file
        klass.bind_template();
        // TODO define actions (no async actions here)
    }
}

// main GObject trait
impl ObjectImpl for MDotEditionPanel {
    fn constructed(&self) {
        // parent constructor, always
        self.parent_constructed();
    }
}

// core widget trait
impl WidgetImpl for MDotEditionPanel {}

// Box subclassing
impl BoxImpl for MDotEditionPanel {}
