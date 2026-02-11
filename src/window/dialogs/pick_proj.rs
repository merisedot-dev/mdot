use adw::subclass::prelude::ObjectSubclassIsExt;
use gettextrs::gettext;
use gtk::{FileDialog, gio::prelude::FileExt, glib::Variant};

use crate::window::Window;

pub const PICKPROJ_NAME: &'static str = "win.pick_proj";

pub async fn pickproj_dialog(caller: Window, _: String, _: Option<Variant>) {
    // fetch dialog
    let dialog = FileDialog::builder()
        .title(gettext("__Projectlocation"))
        .accept_label(gettext("__Choose"))
        .modal(true)
        .build();

    // fetch directory info with the popup
    if let Ok(file) = dialog.select_folder_future(Some(&caller)).await {
        // set project file information
        caller
            .imp()
            .project
            .borrow_mut()
            .set_path(file.path().unwrap_or_default());
    }
}
