use adw::subclass::prelude::ObjectSubclassIsExt;
use gettextrs::gettext;
use gtk::{
    FileDialog, FileFilter,
    gio::{Cancellable, ListStore, prelude::FileExt},
    glib::Variant,
};
use stag::graph::Graph;

use crate::{
    constants::{PROJ_FILE_EXTENSION, WORKS_SCREEN_NAME},
    window::Window,
};

pub const OPEN_NAME: &'static str = "win.open";

pub async fn open_dialog(caller: Window, _: String, _: Option<Variant>) {
    // build filters
    let filters = ListStore::new::<FileFilter>();
    let proj_filter = FileFilter::new();
    proj_filter.add_suffix(format!("*.{}", PROJ_FILE_EXTENSION).as_str());
    proj_filter.set_name(Some(gettext("__MDOTProject").as_str()));
    filters.append(&proj_filter);

    // build dialog
    let dialog = FileDialog::builder()
        .title(gettext("__Selectproject"))
        .filters(&filters)
        .accept_label(gettext("__Open"))
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

        // load project basic info
        let proj_data = caller.imp().project.borrow_mut();
        proj_data.set_name(match file.parse_name().split_once(".") {
            Some((val, _)) => val,
            None => {
                tracing::error!("Project file should have a name");
                return; // STILL... THE FUCK
            }
        });

        // TODO read file contents
        // TODO find how to write that damn graph

        // edit visible stack page
        caller.set_screen(WORKS_SCREEN_NAME);
    }
}
