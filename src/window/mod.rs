mod actions;
mod dialogs;
mod imp;
pub(crate) mod project;

use adw::{Application, subclass::prelude::ObjectSubclassIsExt};
use gtk::{
    gio::{self, Settings},
    glib::{self, Object, object::ObjectExt},
    prelude::{EditableExt, WidgetExt},
};

use crate::{constants::NEWPROJ_SCREEN_NAME, utils::app_id};

glib::wrapper! {
    pub struct MDotWindow(ObjectSubclass<imp::MDotWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow,
                 gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Native,
                    gtk::Root, gtk::ShortcutManager;
}

// Setup methods and building utilities
impl MDotWindow {
    /// custom constructor to ensure we can have a link to our app
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    /// Setting up window default values. It is meant to be called at launch and
    /// only at application launch. This will init values only and only if a
    /// default doesn't exist at creation.
    fn set_defaults(&self) {
        // load default app screen
        self.set_screen(NEWPROJ_SCREEN_NAME);
    }

    /// Fetches settings from GSchema and loads it in the application window. If
    /// it doesn't work, crashes the whole application by precaution.
    fn set_settings(&self) {
        let settings = Settings::new(app_id());
        self.imp()
            .settings
            .set(settings)
            .expect("settings should have been set already");
    }

    /// Binds properties related to [MDotWindow] to subwidgets. This is meant to
    /// be called only during construction
    fn set_bindings(&self) {
        self.imp()
            .drawing_board
            .bind_property("project", self, "project")
            .bidirectional()
            .build();
    }
}

// fetchers and other attribute calculators
impl MDotWindow {
    /// Fetches the current selected conversion core for script purposes.
    ///
    /// **Warning** : in case of errors in the template, this may return an invalid
    /// conversion core name, leading to errors.
    pub fn get_selected_core(&self) -> String {
        match self.imp().core_toggle.active_name() {
            Some(val) => val.to_string(),
            None => String::new(),
        }
    }
}

// logic-related methods
impl MDotWindow {
    /// Changes the displayed window title in the header bar. This should never
    /// be a blank name for usability reason. If that happens, please revert
    /// back to the APP_NAME.
    pub fn set_app_title(&self, name: impl ToString) {
        self.imp().app_title.set_title(&name.to_string());
    }

    /// Changes the displayed windo subtitle in the header bar. This can be a
    /// blank name (unlike [MDotWindow::set_app_title]).
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
        // clear widget contents
        self.imp().path_lbl.set_label("");
        self.imp().proj_name.set_text("");
        self.imp().form_error_lbl.set_text("");
        // clear CSS
        self.clear_form_css();
    }

    /// Clears CSS from the project creation form. This does not alter the
    /// contents of said form.
    pub fn clear_form_css(&self) {
        self.imp().path_picker.remove_css_class("form_err");
        self.imp().proj_name.remove_css_class("form_err");
    }
}
