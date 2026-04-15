use std::{fs::File, io::Write, ops::Deref};

use adw::subclass::prelude::ObjectSubclassIsExt;
use serde_json::{json, to_string};
use tracing::{error, info};

use crate::{
    utils::{MDotActable, MDotAction},
    window::Window,
};

pub struct SaveAction;

impl MDotActable for SaveAction {
    type InnerCallerType = Window;
}

impl MDotAction for SaveAction {
    fn name(&self) -> &'static str {
        "win.save"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        // prefect project info
        let project = caller.imp().project.borrow();
        let graph = project.imp().data.borrow().graph.clone();
        let core = project.imp().data.borrow().core.clone();

        // open file for writing (and delete what was there)
        let mut file = match File::create(project.filepath()) {
            Ok(file) => file,
            Err(why) => {
                error!("{:?}", why);
                return; // no use continuing
            }
        };

        // write project file out
        match file.write_all(
            to_string(&json!({
                "graph": graph.borrow().deref(),
                "core": core.borrow().name()
            }))
            .unwrap_or_default()
            .as_bytes(),
        ) {
            Ok(_) => info!("Saved project file"),
            Err(why) => {
                error!("{:?}", why);
                return;
            }
        }
    }
}
