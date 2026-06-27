//! Gather all drawing functions for each graph elements
mod line;
mod tables;

use gtk::cairo::Context;
use stag::graph::Graph;

/// Embeds a [gtk::cairo] draw function into a drawable object.
/// This trait won't tinker with things like WeakRef to avoid turning it
/// into a mess.
pub trait Drawable {
    /// Draws the [Drawable] using the given context information. Please do not
    /// use borrowed data in there (notably using `RefCell::borrow()`), but try
    /// to snapshot it beforehand.
    fn draw(&self, graph: Graph, ctx: &Context, height: i32, width: i32);
}
