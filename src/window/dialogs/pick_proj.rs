use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    FileDialog, FileFilter,
    gio::{ListStore, prelude::FileExt},
};

use crate::{
    utils::{MDotActable, MDotDialog},
    window::Window,
};

#[derive(Clone, Copy)]
pub struct PickProjDialog;

impl MDotActable for PickProjDialog {
    type InnerCallerType = Window;
}

impl MDotDialog for PickProjDialog {
    fn name(&self) -> &'static str {
        "win.pick_proj"
    }

    async fn handle_dialog(&self, caller: Self::InnerCallerType) {
        // build filters
        let filters = ListStore::new::<FileFilter>();
        let proj_filter = FileFilter::new();
        proj_filter.add_suffix(".mrsdproj");
        filters.append(&proj_filter);

        // build dialog popup
        let dialog = FileDialog::builder()
            .title("Choose a Project")
            .accept_label("Open")
            .filters(&filters)
            .build();

        if let Ok(file) = dialog.open_future(Some(&caller.clone())).await {
            // save to MDot project
            caller
                .imp()
                .project
                .borrow_mut()
                .set_path(file.path().unwrap_or_default());
        }
    }
}
