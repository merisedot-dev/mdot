use adw::subclass::prelude::ObjectSubclassIsExt;
use tracing::error;

use crate::{
    utils::{MDotActable, MDotAction},
    window::MDotWindow,
};

pub struct NewEntityAction;

impl MDotActable for NewEntityAction {
    type InnerCallerType = MDotWindow;
}

impl MDotAction for NewEntityAction {
    fn name(&self) -> &'static str {
        "win.new_entity"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        // prefetch
        let project = caller.imp().project.borrow();

        // create new entity in graph with phony name
        let name = format!("entity_{}", project.get_graph().get_entities().len());
        if let Err(why) = project.edt_graph().mk_entity(&name) {
            error!("{:?}", why);
            return;
        }

        // TODO add entity design to cairo drawing space
        // TODO find where to add the design

        // TODO change focus to the newly built entity
        // TODO update edition panel and graphtree
    }
}
