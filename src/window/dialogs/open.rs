use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    FileDialog, FileFilter,
    gio::{Cancellable, ListStore, prelude::FileExt},
    glib::Variant,
};

use crate::{constants::WORKS_SCREEN_NAME, window::Window};

pub const OPEN_NAME: &'static str = "win.open";

pub async fn open_dialog(caller: Window, _: String, _: Option<Variant>) {
    // build filters
    let filters = ListStore::new::<FileFilter>();
    let proj_filter = FileFilter::new();
    proj_filter.add_suffix("*.mrsproj");
    proj_filter.set_name(Some("MDOT Project"));
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
        // fetch informations from project file
        let content = match file.read(Cancellable::NONE) {
            Ok(content) => content,
            _ => {
                // no file info, returning
                return;
            }
        };
        // TODO load project data
        // edit visible stack page
        caller
            .imp()
            .page_stack
            .set_visible_child_name(WORKS_SCREEN_NAME);
    }
}
