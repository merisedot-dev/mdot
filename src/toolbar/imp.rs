use adw::subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass};
use gtk::{
    CompositeTemplate,
    glib::{self, subclass::InitializingObject},
    subclass::prelude::*,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedot/mdot/toolbar.ui")]
pub struct MDotToolbar {}

#[glib::object_subclass]
impl ObjectSubclass for MDotToolbar {
    const NAME: &'static str = "MDotToolbar";
    type Type = super::Toolbar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        // link the template file
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// GObject core trait
impl ObjectImpl for MDotToolbar {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

// GTK widget core trait
impl WidgetImpl for MDotToolbar {}

// GTKBox override
impl BoxImpl for MDotToolbar {}
