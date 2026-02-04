use adw::{HeaderBar, SplitButton, subclass::prelude::*};
use gtk::{
    Button, CompositeTemplate, MenuButton, Stack,
    glib::{self, subclass::InitializingObject},
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/window.ui")]
pub struct Window {
    // TODO logic-related elements(like settings)

    // template macro components
    #[template_child]
    pub header: TemplateChild<HeaderBar>,
    #[template_child]
    pub page_stack: TemplateChild<Stack>,

    // template button controls
    #[template_child]
    pub menu_btn: TemplateChild<MenuButton>,
    #[template_child]
    pub newfile_btn: TemplateChild<Button>,
    #[template_child]
    pub open_btn: TemplateChild<SplitButton>,
    #[template_child]
    pub undo_btn: TemplateChild<Button>,
    #[template_child]
    pub redo_btn: TemplateChild<Button>,
}

// subclassing our window
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MDotWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // link the template file to our window class
        klass.bind_template();
        // TODO add actions
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template(); // launching the window from template
    }
}

// gobject core trait
impl ObjectImpl for Window {
    fn constructed(&self) {
        // super() call
        self.parent_constructed();
        // inner setup
    }
}

// gtk widget core trait
impl WidgetImpl for Window {}

// window core traits
impl WindowImpl for Window {
    fn close_request(&self) -> glib::Propagation {
        // pass saved thingies to parent class
        self.parent_close_request()
    }
}

// app window inheritance
impl ApplicationWindowImpl for Window {}

// adwaita ApplicationWindow inheritance
impl AdwApplicationWindowImpl for Window {}
