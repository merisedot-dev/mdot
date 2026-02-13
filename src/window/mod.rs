mod actions;
mod dialogs;
mod imp;

use adw::{Application, subclass::prelude::ObjectSubclassIsExt};
use gtk::{
    gio::{self},
    glib::{self, Object},
    prelude::EditableExt,
};

use crate::constants::DEFAULT_SCREEN_NAME;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::MDotWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow,
                 gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Native,
                    gtk::Root, gtk::ShortcutManager;
}

// Setup methods
impl Window {
    /// custom constructor to ensure we can have a link to our app
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }
}

// logic-related methods
impl Window {
    /// Setting up window default values. It is meant to be called at launch.
    fn set_defaults(&self) {
        // load default app screen
        self.set_screen(DEFAULT_SCREEN_NAME);
    }

    pub fn set_screen(&self, name: impl ToString) {
        self.imp()
            .page_stack
            .set_visible_child_name(name.to_string().as_str());
    }

    pub fn clear_form(&self) {
        self.imp().path_lbl.set_label(""); // no more path label
        self.imp().proj_name.set_text(""); // no more project name
    }
}
