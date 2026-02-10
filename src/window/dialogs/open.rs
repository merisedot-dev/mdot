use gtk::{FileDialog, FileFilter, gio::ListStore, glib::Variant};

use crate::window::Window;

pub const OPEN_NAME: &'static str = "win.open";

pub async fn open_dialog(caller: Window, _: String, _: Option<Variant>) {
    // build filters
    let filters = ListStore::new::<FileFilter>();
    let proj_filter = FileFilter::new();
    proj_filter.add_suffix("*.mrsproj");
    filters.append(&proj_filter);

    // build dialog
    let dialog = FileDialog::builder()
        .title("Open project")
        .filters(&filters)
        .accept_label("Open")
        .modal(true)
        .build();

    // call upon file dialog
    if let Ok(file) = dialog.open_future(Some(&caller)).await {
        // TODO fetch informations
        // TODO edit visible stack page
    }
}
