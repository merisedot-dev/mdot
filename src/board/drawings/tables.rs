use gtk::cairo::Context;
use stag::graph::Graph;

use crate::board::{drawings::Drawable, item::DrawnItem};

impl Drawable for DrawnItem {
    fn draw(&self, graph: Graph, ctx: &Context, height: i32, width: i32) {
        // TODO define drawing process
        // TODO test scenario for this
    }
}
