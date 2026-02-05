mod imp;

use adw::{Application, subclass::prelude::ObjectSubclassIsExt};
use gtk::{
    gio,
    gio::Settings,
    glib::{self, Object},
};

use crate::{config::app_id, constants::DEFAULT_SCREEN_NAME};

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

    /// Settings loader for our application, it is meant to be called only once
    /// during window startup.
    fn setup_settings(&self) {
        let settings = Settings::new(app_id());
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` shouldn't be set already");
    }
}

// logic-related methods
impl Window {
    /// Setting up window default values. It is meant to be called at launch.
    fn set_defaults(&self) {
        // load default app screen
        self.imp()
            .page_stack
            .set_visible_child_name(DEFAULT_SCREEN_NAME);
    }
}
