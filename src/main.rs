pub(crate) mod config;
pub(crate) mod window;

use adw::Application;
use gtk::{
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib,
    prelude::GtkWindowExt,
};

use crate::window::Window;

fn main() -> glib::ExitCode {
    // building app
    let app = Application::builder()
        .application_id("com.github.merisedotdev.mdot")
        .build();

    // connect app to signals
    app.connect_activate(|app| {
        let window = Window::new(app);
        window.present();
    });

    // run the application
    app.run()
}
