use adw::subclass::prelude::ObjectSubclassIsExt;
use gettextrs::gettext;
use gtk::{FileDialog, gio::prelude::FileExt, glib::Variant};
use tracing::info;

use crate::window::MDotWindow;

pub const PICKPROJ_NAME: &'static str = "win.pick_proj";

pub async fn pickproj_dialog(caller: MDotWindow, _: String, _: Option<Variant>) {
    // fetch dialog
    let dialog = FileDialog::builder()
        .title(gettext("__Projectlocation"))
        .accept_label(gettext("__Choose"))
        .modal(true)
        .build();

    // fetch directory info with the popup
    if let Ok(file) = dialog.select_folder_future(Some(&caller)).await {
        let proj_filepath = match file.path() {
            Some(path) => path,
            _ => return, // no use continuing
        };

        // set project file information
        caller
            .imp()
            .project
            .borrow()
            .set_path(proj_filepath.clone());
        info!("Fetched project folder");

        // tweak file picker label
        caller
            .imp()
            .path_lbl
            .set_text(proj_filepath.to_str().unwrap_or_default());
        info!("Set temp project folder path");
    }
}
