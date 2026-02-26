use cucumber::{given, then, when};
use stag::entity::{Entity, EntityAttr};

use crate::MDotWorld;

#[given(expr = "a new entity named \"{word}\"")]
#[when(expr = "we build a new entity named \"{word}\"")]
fn mk_entity(world: &mut MDotWorld, name: String) {
    world.entity = Entity::new(name);
}

#[given(expr = "a new attribute \"{word}\" of type {word}")]
fn mk_attr(world: &mut MDotWorld, attr: String, attrtype: String) {
    world.attribute = EntityAttr::try_from(attrtype).unwrap();
    world.attr_name = attr;
}

#[given(expr = "the entity has an attribute \"{word}\" of type {word}")]
fn ensure_entity_attr(world: &mut MDotWorld, attr: String, attrtype: String) {
    world
        .entity
        .add_attr(attr, EntityAttr::try_from(attrtype).unwrap())
        .unwrap();
}

#[when("we add the attribute in the entity")]
fn add_attr(world: &mut MDotWorld) {
    world
        .entity
        .add_attr(world.attr_name.clone(), world.attribute.clone())
        .unwrap();
}

#[when(expr = "the attribute \"{word}\" is deleted from the entity")]
fn del_attr(world: &mut MDotWorld, attr: String) {
    world.entity.del_attr(attr).unwrap();
}

#[then(expr = "the entity is named \"{word}\"")]
fn check_entity_name(world: &mut MDotWorld, name: String) {
    assert_eq!(world.entity.name(), name)
}

#[then(expr = "the entity has {int} attributes")]
fn check_entity_nb_attr(world: &mut MDotWorld, nb: usize) {
    assert_eq!(world.entity.get_all_attrs().len(), nb)
}

#[then(expr = "the entity has an attribute named \"{word}\"")]
fn check_has_attr(world: &mut MDotWorld, attr: String) {
    world.entity.get_attr(attr).unwrap();
}

#[then(expr = "the entity doesn't have an attribute named \"{word}\"")]
fn check_not_attr(world: &mut MDotWorld, attr: String) {
    match world.entity.get_attr(attr) {
        Ok(_) => panic!("This should not happen"),
        Err(_) => { /* This is fine */ }
    }
}

#[then(expr = "the attribute \"{word}\" is of type {word}")]
fn check_attr_type(world: &mut MDotWorld, attr: String, attrtype: String) {
    assert_eq!(
        world.entity.get_attr(attr).unwrap().clone(),
        EntityAttr::try_from(attrtype).unwrap()
    )
}
