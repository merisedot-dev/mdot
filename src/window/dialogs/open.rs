use adw::subclass::prelude::ObjectSubclassIsExt;
use gettextrs::gettext;
use gtk::{
    FileDialog, FileFilter,
    gio::{
        Cancellable, ListStore,
        prelude::{FileExt, InputStreamExtManual},
    },
    glib::{Priority, Variant},
};
use serde_json::from_str;
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

        // load project basic file info
        let proj_name = match file.parse_name().split_once(".") {
            Some((val, _)) => val.to_string(),
            None => {
                tracing::error!("Project file should have a name");
                return; // THE FUCK
            }
        };

        // fetch file folder path
        let proj_path = match file.path() {
            Some(val) => val,
            None => {
                tracing::error!("Project file should have a path...");
                return; // no use
            }
        };
        // filter from raw path
        let mut folder_path = proj_path.components();
        folder_path.next_back();

        // read file contents and turn them into a project graph
        let buf: Vec<u8> = Vec::new();
        let graph = match content.read_all_future(buf, Priority::HIGH).await {
            Ok((val, _, _)) => match String::from_utf8(val) {
                Ok(val_str) => match from_str::<Graph>(&val_str) {
                    Ok(graph) => graph,
                    Err(why) => {
                        tracing::error!("{:?}", why);
                        return; // no use
                    }
                },
                Err(why) => {
                    tracing::error!("{:?}", why);
                    return; // no use continuing
                }
            },
            Err(why) => {
                tracing::error!("{:?}", why);
                return; // no use continuing
            }
        };

        // edit project info
        caller
            .imp()
            .project
            .borrow_mut()
            .imp()
            .data
            .borrow_mut()
            .graph
            .replace(graph);
        caller.imp().project.borrow_mut().set_name(proj_name);
        caller
            .imp()
            .project
            .borrow_mut()
            .set_path(folder_path.as_path());

        // edit visible stack page
        caller.set_screen(WORKS_SCREEN_NAME);
    }
}
