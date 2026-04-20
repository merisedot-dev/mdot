use gtk::{
    CompositeTemplate,
    glib::{self, subclass::InitializingObject},
    subclass::prelude::*,
};
use tracing::info;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/toolbar.ui")]
pub struct MDotToolbar {}

// core subclassing trait for GTK
#[glib::object_subclass]
impl ObjectSubclass for MDotToolbar {
    const NAME: &'static str = "MDotToolbar";
    type Type = super::MDotToolbar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        // link the template to the custom widget
        klass.bind_template();
        info!("Loaded MDotToolbar template");
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template(); // build from template
    }
}

// core object trait, required for subclassing
impl ObjectImpl for MDotToolbar {}

// Main GTK widget related trait
impl WidgetImpl for MDotToolbar {}

// overriding GtkBox behavior
impl BoxImpl for MDotToolbar {}
