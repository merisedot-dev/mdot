use gtk::{
    CompositeTemplate,
    glib::{self, subclass::InitializingObject},
    subclass::prelude::*,
};
use tracing::info;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/mdot_graphrow.ui")]
pub struct MDotGraphRow {}

#[glib::object_subclass]
impl ObjectSubclass for MDotGraphRow {
    const NAME: &'static str = "MDotGraphRow";
    type Type = super::MDotGraphRow;
    type ParentType = gtk::ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        // fetching info from GTK builder template
        klass.bind_template();
        info!("Loaded information for MDotGraphRow");
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template(); // load from template
    }
}

// Core GTK object trait implementation
impl ObjectImpl for MDotGraphRow {}

// Basic widget implementation
impl WidgetImpl for MDotGraphRow {}

// overriding GtkListBoxRow parent type
impl ListBoxRowImpl for MDotGraphRow {}
