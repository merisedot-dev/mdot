mod actions;
mod dialogs;
mod imp;

use adw::{Application, subclass::prelude::ObjectSubclassIsExt};
use gtk::{
    gio::{self},
    glib::{self, Object},
    prelude::EditableExt,
};

use crate::constants::NEWPROJ_SCREEN_NAME;

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

// fetchers and other calculators
impl Window {
    /// Fetches the current selected conversion core for script purposes.
    pub fn get_selected_core(&self) -> String {
        match self.imp().core_toggle.active_name() {
            Some(val) => val.to_string(),
            None => String::new(),
        }
    }
}

// logic-related methods
impl Window {
    /// Setting up window default values. It is meant to be called at launch and
    /// only at application launch.
    fn set_defaults(&self) {
        // load default app screen
        self.set_screen(NEWPROJ_SCREEN_NAME);
    }

    /// Changes the displayed window title in the header bar. This should never
    /// be a blank name for usability reason. If that happens, please revert
    /// back to [APP_NAME].
    pub fn set_app_title(&self, name: impl ToString) {
        self.imp().app_title.set_title(&name.to_string());
    }

    /// Changes the displayed windo subtitle in the header bar. This can be a
    /// blank name (unlike [Window::set_app_title]).
    pub fn set_app_subtitle(&self, name: impl ToString) {
        self.imp().app_title.set_subtitle(&name.to_string());
    }

    /// Change the displayed screen. This may come with logic checks to avoid
    /// breaking user experience. Please call this method to avoid problems.
    pub fn set_screen(&self, name: impl ToString) {
        self.imp()
            .page_stack
            .set_visible_child_name(name.to_string().as_str());
    }

    /// Changes the error label in the project creation form.
    pub fn show_form_err(&self, txt: impl ToString) {
        self.imp().form_error_lbl.set_text(&txt.to_string());
    }

    /// Remove all information from the project form screen. This is not
    /// exhaustive and can be extended should new widgets be added to the screen.
    pub fn clear_form(&self) {
        self.imp().path_lbl.set_label(""); // no more path label
        self.imp().proj_name.set_text(""); // no more project name
        self.imp().form_error_lbl.set_text(""); // no more error
    }
}
