mod graphrow;
mod imp;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use stag::graph::Graph;

use crate::{
    constants::{ENTITY_ICON_NAME, GRAPHLINK_ICON_NAME},
    mdtree::graphrow::MDotGraphRow,
};

glib::wrapper! {
    pub struct MDotGraphTree(ObjectSubclass<imp::MDotGraphTree>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Root;
}

// Logic implementation for behavior of the MDotGraphTree widget.
impl MDotGraphTree {
    /// Loads the given project [Graph] into the widget's GtkListBox. This will
    /// show only the entities and graphlinks of the project... As well as some...
    /// Unique constraints.
    pub fn show_project(&self, graph: Graph) {
        // entities conversion
        for (_, ent) in graph.get_entities() {
            self.imp()
                .elems_list
                .append(&MDotGraphRow::new(ent.name(), ENTITY_ICON_NAME));
        }
        // graphlinks conversion
        for ent in graph.get_lks().iter().map(|(_, v)| v.inner.clone()) {
            self.imp()
                .elems_list
                .append(&MDotGraphRow::new(ent.name(), GRAPHLINK_ICON_NAME));
        }
    }
}
