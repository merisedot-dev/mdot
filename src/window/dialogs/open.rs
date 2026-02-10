use gtk::glib::Variant;

use crate::window::Window;

pub const OPEN_NAME: &'static str = "win.open";

pub async fn open_dialog(caller: Window, _: String, _: Option<Variant>) {
    // TODO build dialog
}
