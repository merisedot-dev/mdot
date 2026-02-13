pub(crate) mod config;
pub(crate) mod constants;
pub(crate) mod edition;
pub(crate) mod project;
pub(crate) mod utils;
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
    config::*,
    utils::{i18n_init, load_css},
    window::Window,
};

fn main() -> glib::ExitCode {
    // logs setup (because logs are good)
    tracing_subscriber::fmt().init();
    info!("Setting up logs");
    i18n_init(); // gettext setup
    info!("Loading up locales");

    // building app with resources
    let res = Resource::load(resources_file()).expect("Could not load resource");
    resources_register(&res);
    let app = Application::builder().application_id(app_id()).build();

    // startup watchdog
    app.connect_startup(|_| {
        info!("startup");
        load_css();
        info!("Loaded css");
    });

    // activation watchdog
    app.connect_activate(|app| {
        info!("activate");
        let window = Window::new(app);
        info!("Loaded main window");
        window.present();
    });

    // run the application
    app.run()
}
