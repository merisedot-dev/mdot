use adw::subclass::prelude::ObjectSubclassIsExt;

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
        // create new entity in graph with phony name
        let project = caller.imp().project.borrow();

        // TODO work out cairo design handling
        // TODO change focus to the newly built entity
        // TODO update edition panel and graphtree
    }
}
