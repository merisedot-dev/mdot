//! Configuration information for the app

// environment values
pub const APP_ID: Option<&str> = option_env!("APP_ID");
pub const RESOURCES_FILE: Option<&str> = option_env!("RESOURCES_FILE");
pub const GETTEXT_PACKAGE: Option<&str> = option_env!("GETTEXT_PACKAGE");
pub const LOCALEDIR: Option<&str> = option_env!("LOCALEDIR");

// env variable fetchers
pub fn app_id() -> &'static str {
    APP_ID.expect("APP_ID not set at compile time")
}

pub fn resources_file() -> &'static str {
    RESOURCES_FILE.expect("RESOURCES_FILE not set at compile time")
}

pub fn gettext_package() -> &'static str {
    GETTEXT_PACKAGE.expect("GETTEXT_PACKAGE not set at compile time")
}

pub fn localedir() -> &'static str {
    LOCALEDIR.expect("LOCALEDIR not set at compile time")
}
