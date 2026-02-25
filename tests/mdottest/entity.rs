use cucumber::{given, then, when};
use stag::entity::Entity;

use crate::MDotWorld;

#[given(expr = "a new entity named\"{word}\"")]
#[when(expr = "we build a new entity named \"{word}\"")]
fn mk_entity(world: &mut MDotWorld, name: String) {
    world.entity = Entity::new(name);
}

#[then(expr = "the entity is named \"{word}\"")]
fn check_entity_name(world: &mut MDotWorld, name: String) {
    assert_eq!(world.entity.name(), name)
}

#[then(expr = "the entity has {int} attributes")]
fn check_entity_nb_attr(world: &mut MDotWorld, nb: usize) {
    assert_eq!(world.entity.get_all_attrs().len(), nb)
}
