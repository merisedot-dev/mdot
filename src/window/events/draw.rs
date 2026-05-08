use gtk::{DrawingArea, cairo::Context};
use stag::graph::Graph;

/// Draws the entire [Graph] on screen for the user. This will also link the drawn
/// items to an `on_click` event (for MDotPanel use). Only those items will be
/// linked to an `on_click` :
/// - Entity
/// - GrapLink's inner Entity
///
/// **Warning**: In case of aberrations in the [Graph], some graphical
/// inconsistencies may occur.
pub fn draw_graph(drawing: &DrawingArea, context: &Context, graph: Graph) {
    // TODO define how to draw an entity
    // TODO define how to draw a graphlink
    // TODO draw everything
    // TODO make everything clickable (for panel usage)
    // TODO ensure everything draw is linked to a graph item
}
