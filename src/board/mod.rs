mod imp;
mod item;

use gtk::glib;
use stag::graph::Graph;

glib::wrapper! {
    pub struct MDotDrawingBoard(ObjectSubclass<imp::MDotDrawingBoard>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Root;
}

// logic implementation
impl MDotDrawingBoard {
    pub fn draw_graph(&self, graph: Graph) {
        // TODO find out how to draw
        // TODO call this
    }
}
