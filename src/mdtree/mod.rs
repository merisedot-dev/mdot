mod graphrow;
mod imp;

use gtk::glib;
use stag::graph::Graph;

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
        // TODO define list row
    }
}
