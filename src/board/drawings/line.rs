use gtk::cairo::Context;
use stag::graph::Graph;

use crate::board::{drawings::Drawable, lines::DrawnLine};

impl Drawable for DrawnLine {
    fn draw(&self, graph: Graph, ctx: &Context, height: i32, width: i32) {
        // TODO draw a single line between two entities
    }
}
