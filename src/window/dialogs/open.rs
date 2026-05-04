use std::io::Read;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gettextrs::gettext;
use gtk::{
    FileDialog,
    gio::{
        Cancellable,
        prelude::{FileExt, InputStreamExtManual},
    },
    glib::Variant,
};
use serde_json::{Value, from_str, from_value};
use stag::{graph::Graph, script::ExposedCore};
use tracing::{error, info};

use crate::{constants::WORKS_SCREEN_NAME, window::MDotWindow};

pub const OPEN_NAME: &'static str = "win.open";

pub async fn open_dialog(caller: MDotWindow, _: String, _: Option<Variant>) {
    // build dialog
    let dialog = FileDialog::builder()
        .title(gettext("__Selectproject"))
        .accept_label(gettext("__Open"))
        .modal(true)
        .build();
    info!("Searching for project file...");

    // call upon file dialog
    if let Ok(file) = dialog.open_future(Some(&caller)).await {
        info!("Picked project file");
        // load project basic file info
        let proj_name = match file.parse_name().split("/").last() {
            Some(val) => val.to_string(),
            None => {
                error!("Project file should have a name");
                return; // THE FUCK
            }
        };
        // fetch file folder path
        let proj_path = match file.path() {
            Some(val) => val,
            None => {
                error!("Project file should have a path...");
                return; // no use
            }
        };
        // filter from raw path
        let mut folder_path = proj_path.components();
        folder_path.next_back();

        // read the actual file contents and parse them
        let mut proj_file = match file.read(Cancellable::NONE) {
            Ok(val) => val.into_read(),
            Err(why) => {
                error!("{:?}", why);
                return;
            }
        };
        let mut contents: Vec<u8> = Vec::new();
        let json_text = match proj_file.read_to_end(&mut contents) {
            Ok(_) => match String::from_utf8(contents) {
                Ok(val) => match from_str::<Value>(&val) {
                    Ok(json_val) => json_val,
                    Err(why) => {
                        error!("{:?}", why);
                        caller.show_form_err(gettext("__MalformedFile"));
                        return;
                    }
                },
                Err(why) => {
                    error!("{:?}", why);
                    caller.show_form_err(gettext("__FileReadError"));
                    return;
                }
            },
            Err(why) => {
                error!("{:?}", why);
                caller.show_form_err(gettext("__FileReadError"));
                return;
            }
        };

        // Parsing conversion core from json info
        let core = match json_text.get("core") {
            Some(val) => match val {
                Value::String(str_val) => ExposedCore::from(str_val.to_string()),
                _ => {
                    caller.show_form_err(gettext("__MalformedFile"));
                    return;
                }
            },
            None => {
                caller.show_form_err(gettext("__MalformedFile"));
                return;
            }
        };

        // parse graph from json info
        let graph = match json_text.get("graph") {
            Some(val) => match from_value::<Graph>(val.clone()) {
                Ok(graph_obj) => graph_obj,
                Err(why) => {
                    error!("{:?}", why);
                    caller.show_form_err(gettext("__MalformedFile"));
                    return;
                }
            },
            None => {
                caller.show_form_err(gettext("__MalformedFile"));
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
        caller.imp().project.borrow_mut().set_core(core);
        caller.imp().project.borrow_mut().set_graph(graph);

        // Logging
        info!("Fetched project file info");

        // edit visible stack page and display tweaks
        caller.clear_form();
        caller.set_screen(WORKS_SCREEN_NAME);
        caller.set_app_title(&proj_name);
        caller.set_app_subtitle(proj_path.to_str().unwrap_or_default());
    }
}
