use cucumber::{given, then, when};
use stag::entity::{EntityAttr, GraphLink};

use crate::MDotWorld;

#[given(expr = "the GraphLink has {int} attributes")]
fn ensure_nb_attrs(world: &mut MDotWorld, nb: usize) {
    for i in 0..nb {
        world
            .link
            .inner
            .add_attr(format!("att_{}", i), EntityAttr::INTEGER)
            .unwrap();
    }
}

#[given(expr = "a new GraphLink named \"{word}\"")]
#[when(expr = "we build a new GraphLink named \"{word}\"")]
fn mk_link(world: &mut MDotWorld, name: String) {
    world.link = GraphLink::new(name);
}

#[when("we add the attribute to the GraphLink")]
fn mk_attr(world: &mut MDotWorld) {
    world
        .link
        .inner
        .add_attr(world.attr_name.clone(), world.attribute.clone())
        .unwrap();
}

#[then(expr = "the GraphLink is named \"{word}\"")]
fn check_name(world: &mut MDotWorld, name: String) {
    assert_eq!(world.link.inner.name(), name)
}

#[then(expr = "the GraphLink has {int} attributes")]
fn check_nb_attrs(world: &mut MDotWorld, nb: usize) {
    assert_eq!(world.link.inner.get_all_attrs().len(), nb)
}

#[then(expr = "the GraphLink has an attribute named \"{word}\"")]
fn check_attr(world: &mut MDotWorld, attr: String) {
    world.link.inner.get_attr(attr).unwrap();
}

#[then(expr = "the GraphLink has {int} known entities")]
fn check_nb_entities(world: &mut MDotWorld, nb: usize) {
    assert_eq!(world.link.get_all_lks().len(), nb)
}

#[then(expr = "the GraphLink attribute \"{word}\" is of type {word}")]
fn check_attrtype(world: &mut MDotWorld, attr: String, attrtype: String) {
    let grphlk_attr = EntityAttr::try_from(attrtype).unwrap();
    assert_eq!(
        world.link.inner.get_attr(attr).unwrap().clone(),
        grphlk_attr
    );
}
