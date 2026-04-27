use gettextrs::{LocaleCategory, bind_textdomain_codeset, bindtextdomain, setlocale, textdomain};
use gtk::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, gdk::Display,
    style_context_add_provider_for_display,
};

use crate::utils::{gettext_package, localedir};

/// Loads all translations from the `po` directory. In case of any missing
/// translation file, it will just die. Non-updated translations may occur.
pub fn i18n_init() {
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(gettext_package(), localedir())
        .expect("Unable to bind the merisedot text domain");
    bind_textdomain_codeset(gettext_package(), "UTF-8")
        .expect("Unable to set text domain encoding");
    textdomain(gettext_package()).expect("Unable to switch to text domain");
}

/// Loads all CSS style sheets from packaged data. This should not fail, if that
/// happens, this will crash the whole app as prevention.
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
