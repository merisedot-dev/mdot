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
use serde_json::{Value, de::from_str, from_value};
use stag::{graph::Graph, script::ExposedCore};
use tracing::info;

use crate::{
    constants::{PROJ_FILE_EXTENSION, WORKS_SCREEN_NAME},
    window::MDotWindow,
};

pub const OPEN_NAME: &'static str = "win.open";

pub async fn open_dialog(caller: MDotWindow, _: String, _: Option<Variant>) {
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
    info!("Searching for project file...");

    // call upon file dialog
    if let Ok(file) = dialog.open_future(Some(&caller)).await {
        info!("Picked project file");
        // fetch informations from project file
        let content = match file.read(Cancellable::NONE) {
            Ok(content) => content,
            _ => {
                // no file info, returning
                caller.show_form_err(gettext("__FileReadError"));
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
        let obj = match content.read_all_future(buf, Priority::HIGH).await {
            Ok((val, _, _)) => match String::from_utf8(val) {
                Ok(val_str) => match from_str::<Value>(&val_str) {
                    Ok(json_val) => json_val,
                    Err(why) => {
                        tracing::error!("{:?}", why);
                        caller.show_form_err(gettext("__MalformedFile"));
                        return;
                    }
                },
                Err(why) => {
                    tracing::error!("{:?}", why);
                    caller.show_form_err(gettext("__UnreadableFile"));
                    return; // no use continuing
                }
            },
            Err(why) => {
                tracing::error!("{:?}", why);
                caller.show_form_err(gettext("__UnreadableFile"));
                return; // no use continuing
            }
        };

        // split JSON to extract graph and core
        let graph = match obj.get("graph") {
            Some(val) => match from_value::<Graph>(val.clone()) {
                Ok(graph) => graph,
                Err(why) => {
                    tracing::error!("{:?}", why);
                    caller.show_form_err(gettext("__UnreadableGraph"));
                    return;
                }
            },
            None => return,
        };
        let conversion_core = match obj.get("core") {
            Some(val) => match val {
                Value::String(val_str) => ExposedCore::from(val_str.clone()),
                _ => return, // aberration
            },
            None => {
                caller.show_form_err(gettext("__UnreadableCore"));
                return;
            }
        };

        // edit project info
        caller
            .imp()
            .project
            .borrow_mut()
            .set_name(proj_name.clone());
        caller
            .imp()
            .project
            .borrow_mut()
            .set_path(folder_path.as_path());
        // add graph and core
        caller
            .imp()
            .project
            .borrow_mut()
            .imp()
            .data
            .borrow_mut()
            .graph
            .replace(graph);
        caller
            .imp()
            .project
            .borrow_mut()
            .imp()
            .data
            .borrow_mut()
            .core
            .replace(conversion_core);

        // edit visible stack page and display tweaks
        caller.clear_form();
        caller.set_screen(WORKS_SCREEN_NAME);
        caller.set_app_title(&proj_name);
        caller.set_app_subtitle(proj_path.to_str().unwrap_or_default());
    }
}
