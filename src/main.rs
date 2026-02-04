pub(crate) mod config;

use gtk::{Application, gio::prelude::ApplicationExtManual, glib};

fn main() -> glib::ExitCode {
    // building app
    let app = Application::builder()
        .application_id("com.github.merisedot-dev.mdot")
        .build();

    // run the application
    app.run()
}
