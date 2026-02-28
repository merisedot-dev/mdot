use cucumber::{given, then, when};
use stag::graph::Graph;

use crate::MDotWorld;

#[given(expr = "an entity named \"{word}\" in graph")]
fn ensure_entity(world: &mut MDotWorld, name: String) {
    world.graph.mk_entity(name).unwrap();
}

#[given("a new graph")]
#[when("we build a new graph")]
fn mk_graph(world: &mut MDotWorld) {
    world.graph = Graph::default();
}

#[when("we add the entity to our graph")]
fn slot_entity(world: &mut MDotWorld) {
    world.graph.mk_entity(world.entity.name()).unwrap();
}

#[when(expr = "we link \"{word}\" and \"{word}\" under the name \"{word}\"")]
fn mk_link(world: &mut MDotWorld, e1: String, e2: String, lk: String) {
    world.graph.link(lk, e1, e2).unwrap();
}

#[when(expr = "we add \"{word}\" to the GraphLink \"{word}\"")]
fn ternary(world: &mut MDotWorld, entity: String, glk: String) {
    world.graph.extra_lk(glk,entity).unwrap();
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

#[then(expr = "the graph has a GraphLink named \"{word}\"")]
fn check_link(world: &mut MDotWorld, name: String) {
    world.graph.get_lk(name).unwrap();
}

#[then(expr = "the GraphLink \"{word}\" knows an entity named \"{word}\"")]
fn check_linked(world: &mut MDotWorld, lk: String, ent: String) {
    world.graph.get_lk(lk).unwrap().get_lk(ent).unwrap();
}
