use gtk::{
    ColumnView, CompositeTemplate, Stack,
    glib::{self, subclass::InitializingObject},
    subclass::prelude::*,
};
use tracing::info;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/mdot_panel.ui")]
pub struct MDotPanel {
    #[template_child]
    pub panel_stack: TemplateChild<Stack>,
    #[template_child]
    pub attributes_clmn: TemplateChild<ColumnView>,
    #[template_child]
    pub links_stack: TemplateChild<Stack>,
    #[template_child]
    pub grphlk_clmn: TemplateChild<ColumnView>,
}

#[glib::object_subclass]
impl ObjectSubclass for MDotPanel {
    const NAME: &'static str = "MDotPanel";
    type Type = super::MDotPanel;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        // TODO fetch extra template informations
        // fetch correct building information from template
        klass.bind_template();
        info!("Loaded template for MDotPanel");
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template(); // build widget from template info
    }
}

// Basic override trait
impl ObjectImpl for MDotPanel {
    fn constructed(&self) {
        // super() call
        self.parent_constructed();
        // inner setup
        let obj = self.obj();
        obj.set_defaults();
    }
}

// Changing behavior of GtkWidget
impl WidgetImpl for MDotPanel {}

// Overriding box behavior
impl BoxImpl for MDotPanel {}
