use std::cell::RefCell;

use adw::{SplitButton, subclass::prelude::*};
use gtk::{
    Button, CompositeTemplate, DrawingArea, Entry, Label, MenuButton, Stack,
    glib::{self, clone, subclass::InitializingObject},
};
use tracing::info;

use crate::{
    project::Project,
    utils::*,
    window::{
        actions::mk_actions,
        dialogs::{DialogPopups, mk_dialogs},
    },
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/window.ui")]
pub struct MDotWindow {
    // logic-related elements (like app settings)
    pub project: RefCell<Project>,

    // template macro components
    #[template_child]
    pub page_stack: TemplateChild<Stack>,

    // header controls
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

    // newproj form controls
    #[template_child]
    pub proj_name: TemplateChild<Entry>,
    #[template_child]
    pub path_picker: TemplateChild<Button>,
    #[template_child]
    pub path_lbl: TemplateChild<Label>,

    // workspace controls
    #[template_child]
    pub graph_drawing: TemplateChild<DrawingArea>,
}

// subclassing our window
#[glib::object_subclass]
impl ObjectSubclass for MDotWindow {
    const NAME: &'static str = "MDotWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // link the template file to our window class
        klass.bind_template();
        // installing GActions
        for action in mk_actions() {
            klass.install_action(action.name(), None, move |win, txt, var| {
                // do NOT prefetch implementation
                action.handle_activate(win, txt, var);
            });
        }
        // async actions
        for dialog in mk_dialogs() {
            let dial = match dialog {
                DialogPopups::PICKPROJ(dial) => dial,
            };
            klass.install_action_async(
                dial.clone().name(),
                None,
                clone!(move |caller_window, _, _| async move {
                    dial.handle_dialog(caller_window).await;
                }),
            );
        }
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

// gtk widget core trait
impl WidgetImpl for MDotWindow {}

// window core traits
impl WindowImpl for MDotWindow {
    fn close_request(&self) -> glib::Propagation {
        // pass saved thingies to parent class
        self.parent_close_request()
    }
}

// app window inheritance
impl ApplicationWindowImpl for MDotWindow {}

// adwaita ApplicationWindow inheritance
impl AdwApplicationWindowImpl for MDotWindow {}
