use gtk::{
    CompositeTemplate,
    glib::{self, subclass::InitializingObject},
    subclass::prelude::*,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/merisedotdev/mdot/toolbar.ui")]
pub struct MDotToolbar {}

#[glib::object_subclass]
impl ObjectSubclass for MDotToolbar {
    const NAME: &'static str = "MDotToolbar";
    type Type = super::MDotToolbar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        // link the template to the custom widget
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template(); // build from template
    }
}

impl ObjectImpl for MDotToolbar {}

impl WidgetImpl for MDotToolbar {}

impl BoxImpl for MDotToolbar {}
