pub(crate) mod config;
pub(crate) mod constants;
pub(crate) mod project;
pub(crate) mod utils;
pub(crate) mod window;

use adw::Application;
use gettextrs::{LocaleCategory, bind_textdomain_codeset, bindtextdomain, setlocale, textdomain};
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

use crate::{config::*, window::Window};

fn i18n_init() {
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(gettext_package(), localedir())
        .expect("Unable to bind the merisedot text domain");
    bind_textdomain_codeset(gettext_package(), "UTF-8")
        .expect("Unable to set text domain encoding");
    textdomain(gettext_package()).expect("Unable to switch to text domain");
}

fn main() -> glib::ExitCode {
    // logs setup (because logs are good)
    tracing_subscriber::fmt().init();
    i18n_init();

    // building app with resources
    let res = Resource::load(resources_file()).expect("Could not load resource");
    resources_register(&res);
    let app = Application::builder().application_id(app_id()).build();

    // startup watchdog
    app.connect_startup(|_| {
        info!("startup");
        // TODO maybe we'll need to compile extra interfaces
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
