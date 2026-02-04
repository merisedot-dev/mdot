pub(crate) mod config;
pub(crate) mod window;

use adw::Application;
use gtk::{
    gio::{
        Resource,
        prelude::{ApplicationExt, ApplicationExtManual},
        resources_register,
    },
    glib,
    prelude::GtkWindowExt,
};
use tracing::info;

use crate::{
    config::{app_id, resources_file},
    window::Window,
};

fn main() -> glib::ExitCode {
    // building app with resources
    let res = Resource::load(resources_file()).expect("Could not load resource");
    resources_register(&res);
    let app = Application::builder().application_id(app_id()).build();

    // startup watchdog
    app.connect_startup(|app| {
        info!("startup");
    });

    // activation watchdog
    app.connect_activate(|app| {
        info!("activate");
        let window = Window::new(app);
        window.present();
    });

    // run the application
    app.run()
}
