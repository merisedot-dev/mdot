use std::cell::{OnceCell, RefCell};

use adw::{SplitButton, ToggleGroup, WindowTitle, subclass::prelude::*};
use gtk::{
    Button, CompositeTemplate, DrawingArea, Entry, Label, MenuButton, Stack,
    gio::{Menu, Settings},
    glib::{self, subclass::InitializingObject, types::StaticTypeExt},
};
use tracing::info;

use crate::{
    mdtree::MDotGraphTree,
    panel::MDotPanel,
    window::{actions::mk_actions, dialogs::*, project::Project},
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/merisedotdev/mdot/mdot_window.ui")]
pub struct MDotWindow {
    // logic-related elements
    pub project: RefCell<Project>,
    pub settings: OnceCell<Settings>,
    #[template_child]
    pub proj_menu: TemplateChild<Menu>,
    #[template_child]
    pub graph_tree: TemplateChild<MDotGraphTree>,

    // template macro components
    #[template_child]
    pub page_stack: TemplateChild<Stack>,
    #[template_child]
    pub graph_drawing: TemplateChild<DrawingArea>,
    #[template_child]
    pub edition_panel: TemplateChild<MDotPanel>,

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

    // editor toolbar controls
    #[template_child]
    pub new_entity_btn: TemplateChild<Button>,
    #[template_child]
    pub del_entity_btn: TemplateChild<Button>,
    #[template_child]
    pub new_graphlink_btn: TemplateChild<Button>,
    #[template_child]
    pub del_graphlink_btn: TemplateChild<Button>,
}

// subclassing our window
#[glib::object_subclass]
impl ObjectSubclass for MDotWindow {
    const NAME: &'static str = "MDotWindow";
    type Type = super::MDotWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // Check for any required subtype
        MDotGraphTree::ensure_type();
        MDotPanel::ensure_type();
        info!("Checked for subtemplates");
        // link the template file to our window class
        klass.bind_template();
        info!("Loaded MDotWindow template");
        // installing GActions
        for action in mk_actions() {
            let action_name = action.name();
            klass.install_action(action_name, None, move |win, txt, variant| {
                // do NOT prefetch implementation
                action.handle_activate(win, txt, variant);
            });
            info!("Loaded action {}", action_name);
        }
        // install async actions
        klass.install_action_async(PICKPROJ_NAME, None, pickproj_dialog);
        klass.install_action_async(OPEN_NAME, None, open_dialog);
        info!("Loaded dialogs");
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
        obj.set_defaults();
        obj.set_settings();
        obj.set_handlers();
    }
}

// GTK widget core trait
impl WidgetImpl for MDotWindow {}

// window core traits
impl WindowImpl for MDotWindow {
    fn close_request(&self) -> glib::Propagation {
        // TODO save popup
        // TODO write out settings
        // pass saved thingies to parent class
        self.parent_close_request()
    }
}

// app window inheritance
impl ApplicationWindowImpl for MDotWindow {}

// adwaita ApplicationWindow inheritance
impl AdwApplicationWindowImpl for MDotWindow {}
