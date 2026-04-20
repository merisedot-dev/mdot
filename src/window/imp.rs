use std::cell::RefCell;

use adw::{SplitButton, ToggleGroup, WindowTitle, subclass::prelude::*};
use gtk::{
    Button, CompositeTemplate, DrawingArea, Entry, Label, MenuButton, Stack,
    gio::Menu,
    glib::{self, subclass::InitializingObject, types::StaticTypeExt},
};
use tracing::info;

use crate::{
    project::Project,
    toolbar::MDotToolbar,
    window::{actions::mk_actions, dialogs::*},
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/window.ui")]
pub struct MDotWindow {
    // logic-related elements (like app settings, inner info or menus)
    pub project: RefCell<Project>,
    #[template_child]
    pub proj_menu: TemplateChild<Menu>,
    #[template_child]
    pub toolbar: TemplateChild<MDotToolbar>,

    // template macro components
    #[template_child]
    pub page_stack: TemplateChild<Stack>,
    #[template_child]
    pub graph_drawing: TemplateChild<DrawingArea>,

    // header controls
    #[template_child]
    pub menu_btn: TemplateChild<MenuButton>,
    #[template_child]
    pub open_btn: TemplateChild<SplitButton>,
    #[template_child]
    pub undo_btn: TemplateChild<Button>,
    #[template_child]
    pub redo_btn: TemplateChild<Button>,
    #[template_child]
    pub app_title: TemplateChild<WindowTitle>,

    // newproj form controls
    #[template_child]
    pub proj_name: TemplateChild<Entry>,
    #[template_child]
    pub path_picker: TemplateChild<Button>,
    #[template_child]
    pub path_lbl: TemplateChild<Label>,
    #[template_child]
    pub form_error_lbl: TemplateChild<Label>,
    #[template_child]
    pub core_toggle: TemplateChild<ToggleGroup>,
}

// subclassing our window
#[glib::object_subclass]
impl ObjectSubclass for MDotWindow {
    const NAME: &'static str = "MDotWindow";
    type Type = super::MDotWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // Check for any required subtype
        MDotToolbar::ensure_type();
        // link the template file to our window class
        klass.bind_template();
        // installing GActions
        for action in mk_actions() {
            klass.install_action(action.name(), None, move |win, txt, variant| {
                // do NOT prefetch implementation
                action.handle_activate(win, txt, variant);
            });
        }
        // install async actions
        klass.install_action_async(PICKPROJ_NAME, None, pickproj_dialog);
        klass.install_action_async(OPEN_NAME, None, open_dialog);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template(); // launching the window from template
    }
}

// GObject core trait
impl ObjectImpl for MDotWindow {
    fn constructed(&self) {
        // super() call
        self.parent_constructed();
        // inner setup
        let obj = self.obj();
        info!("Loading default values");
        obj.set_defaults();
    }
}

// GTK widget core trait
impl WidgetImpl for MDotWindow {}

// window core traits
impl WindowImpl for MDotWindow {
    fn close_request(&self) -> glib::Propagation {
        // TODO save popup
        // pass saved thingies to parent class
        self.parent_close_request()
    }
}

// app window inheritance
impl ApplicationWindowImpl for MDotWindow {}

// adwaita ApplicationWindow inheritance
impl AdwApplicationWindowImpl for MDotWindow {}
