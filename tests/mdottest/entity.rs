use cucumber::{then, when};
use stag::entity::Entity;

use crate::MDotWorld;

#[when(expr = "we build a new entity named \"{word}\"")]
fn mk_entity(world: &mut MDotWorld, name: String) {
    world.entity = Entity::new(name);
}

#[then(expr = "the entity is named \"{word}\"")]
fn check_entity_name(world: &mut MDotWorld, name: String) {
    assert_eq!(world.entity.name(), name)
}
