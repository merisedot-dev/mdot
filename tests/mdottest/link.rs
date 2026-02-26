use cucumber::{given, then, when};
use stag::entity::GraphLink;

use crate::MDotWorld;

#[given(expr = "a new GraphLink named \"{word}\"")]
#[when(expr = "we build a new GraphLink named \"{word}\"")]
fn mk_link(world: &mut MDotWorld, name: String) {
    world.link = GraphLink::new(name);
}

#[then(expr = "the GraphLink is named \"{word}\"")]
fn check_name(world: &mut MDotWorld, name: String) {
    assert_eq!(world.link.entity().name(), name)
}

#[then(expr = "the GraphLink has {int} attributes")]
fn check_nb_attrs(world: &mut MDotWorld, nb: usize) {
    assert_eq!(world.entity.get_all_attrs().len(), nb)
}
