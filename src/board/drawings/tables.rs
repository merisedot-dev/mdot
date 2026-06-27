use gtk::cairo::Context;
use stag::graph::Graph;

use crate::board::{drawings::Drawable, item::DrawnItem};

impl Drawable for DrawnItem {
    fn draw(&self, graph: Graph, ctx: &Context, height: i32, width: i32) {
        // prefetch data
        let entity = match graph.get_ent(self.ent_name().to_string()) {
            Ok(val) => val,
            _ => return, // cannot display a non-entity
        };

        // define drawing dimensions
        let t_height = entity.get_all_attrs().len();

        // TODO define drawing process
        // TODO define edge coordinates
        // TODO implement variable attribute size
    }
}
