use gtk::{
    CompositeTemplate, Image, Label,
    glib::{self, Properties, subclass::InitializingObject},
    subclass::prelude::*,
};
use tracing::info;

#[derive(CompositeTemplate, Default, Properties)]
#[properties(wrapper_type = super::MDotGraphRow)]
#[template(resource = "/com/github/merisedotdev/mdot/mdot_graphrow.ui")]
pub struct MDotGraphRow {
    // Information-related properties
    pub it_name: String,
    pub icon_name: String,
    // Child widgets
    #[template_child]
    pub item_icon: TemplateChild<Image>,
    #[template_child]
    pub item_lbl: TemplateChild<Label>,
}

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

#[glib::derived_properties]
impl ObjectImpl for MDotGraphRow {
    fn constructed(&self) {
        // super() call
        self.parent_constructed();
        // inner setup
        self.item_lbl.set_label(&self.it_name);
        self.item_icon.set_icon_name(Some(&self.icon_name));
    }
}

// Basic widget implementation
impl WidgetImpl for MDotGraphRow {}

// overriding GtkListBoxRow parent type
impl ListBoxRowImpl for MDotGraphRow {}
