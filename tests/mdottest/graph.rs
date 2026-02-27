use cucumber::{given, then, when};
use stag::graph::Graph;

use crate::MDotWorld;

#[given("a new graph")]
#[when("we build a new graph")]
fn mk_graph(world: &mut MDotWorld) {
    world.graph = Graph::default();
}

#[when("we add the entity to our graph")]
fn slot_entity(world: &mut MDotWorld) {
    world.graph.mk_entity(world.entity.name()).unwrap();
}

#[then(expr = "the graph has {int} entities")]
fn check_nb_entities(world: &mut MDotWorld, nb: usize) {
    assert_eq!(world.graph.get_entities().len(), nb)
}

#[then(expr = "the graph has {int} links")]
fn check_nb_links(world: &mut MDotWorld, nb: usize) {
    assert_eq!(world.graph.get_lks().len(), nb)
}

#[then(expr = "the graph has an entity named \"{word}\"")]
fn check_entity(world: &mut MDotWorld, entity: String) {
    world.graph.get_entity(entity).unwrap();
}
