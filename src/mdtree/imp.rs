use gtk::{
    CompositeTemplate, ListBox,
    glib::{self, subclass::InitializingObject, types::StaticTypeExt},
    subclass::prelude::*,
};
use tracing::info;

use crate::mdtree::graphrow::MDotGraphRow;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/mdot_tree.ui")]
pub struct MDotGraphTree {
    #[template_child]
    pub elems_list: TemplateChild<ListBox>,
}

#[glib::object_subclass]
impl ObjectSubclass for MDotGraphTree {
    const NAME: &'static str = "MDotGraphTree";
    type Type = super::MDotGraphTree;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        // check extra templates
        MDotGraphRow::ensure_type();
        info!("Fetched extra templates for MDotGraphTree");
        // link the tmplate file to this class
        klass.bind_template();
        info!("Loaded MDotGraphTree template");
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template(); // creating widget from template
    }
}

// core object subclassing trait for a GObject
impl ObjectImpl for MDotGraphTree {}

// overriding GTK widget behavior
impl WidgetImpl for MDotGraphTree {}

// overriding GtkBox behavior
impl BoxImpl for MDotGraphTree {}
