use cucumber::{then, when};
use stag::graph::Graph;

use crate::MDotWorld;

#[when("we build a new graph")]
fn mk_graph(world: &mut MDotWorld) {
    world.graph = Graph::default();
}

#[then(expr = "the graph has {int} entities")]
fn check_nb_entities(world: &mut MDotWorld, nb: usize) {
    assert_eq!(world.graph.get_entities().len(), nb)
}

#[then(expr = "the graph has {int} links")]
fn check_nb_links(world: &mut MDotWorld, nb: usize) {
    assert_eq!(world.graph.get_lks().len(), nb)
}
