use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    FileDialog, FileFilter,
    gio::{ListStore, prelude::FileExt},
    glib::Variant,
};

use crate::window::Window;

pub const PICKPROJ_NAME: &'static str = "win.pick_proj";

pub async fn pickproj_dialog(caller: Window, _: String, _: Option<Variant>) {
    // build filters
    let filters = ListStore::new::<FileFilter>();
    let proj_filter = FileFilter::new();
    proj_filter.add_suffix(".mrsdproj");
    proj_filter.set_name(Some("Project file"));
    filters.append(&proj_filter);

    // build dialog popup
    let dialog = FileDialog::builder()
        .title("Choose a Project")
        .accept_label("Open")
        .modal(true)
        .filters(&filters)
        .build();

    if let Ok(file) = dialog.save_future(Some(&caller.clone())).await {
        // save to MDot project
        caller
            .imp()
            .project
            .borrow_mut()
            .set_path(file.path().unwrap_or_default());
    }
}
