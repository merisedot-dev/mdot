use gettextrs::{LocaleCategory, bind_textdomain_codeset, bindtextdomain, setlocale, textdomain};
use gtk::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, gdk::Display,
    style_context_add_provider_for_display,
};

use crate::config::{gettext_package, localedir};

pub fn i18n_init() {
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(gettext_package(), localedir())
        .expect("Unable to bind the merisedot text domain");
    bind_textdomain_codeset(gettext_package(), "UTF-8")
        .expect("Unable to set text domain encoding");
    textdomain(gettext_package()).expect("Unable to switch to text domain");
}

pub fn load_css() {
    // fetch CSS content
    let provider = CssProvider::new();
    provider.load_from_resource("/com/github/merisedotdev/mdot/mdot.css");

    // slot that CSS in the app
    style_context_add_provider_for_display(
        &Display::default().expect("Could not get access to a display"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
