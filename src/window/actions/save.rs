use std::{fs::File, io::Write, ops::Deref};

use adw::subclass::prelude::ObjectSubclassIsExt;
use serde_json::ser::to_string;
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

        // open file for writing (and delete what was there)
        let mut file = match File::create(project.filepath()) {
            Ok(file) => file,
            Err(why) => {
                error!("{:?}", why);
                return; // no use continuing
            }
        };

        // write project graph info
        match file.write_all(
            match to_string(graph.borrow().deref()) {
                Ok(val) => val,
                Err(why) => {
                    error!("{:?}", why);
                    return;
                }
            }
            .as_bytes(),
        ) {
            Ok(_) => info!("Saved file"),
            Err(why) => error!("{:?}", why),
        };

        // TODO display
    }
}
