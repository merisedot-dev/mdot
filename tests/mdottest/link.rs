use cucumber::{given, then, when};
use stag::entity::{AttrRole, EntityAttr, GraphLink};

use crate::MDotWorld;

#[given(expr = "the GraphLink has {int} attributes")]
fn ensure_nb_attrs(world: &mut MDotWorld, nb: usize) {
    for i in 0..nb {
        world
            .link
            .inner
            .add_attr(
                format!("att_{}", i),
                EntityAttr::INTEGER,
                stag::entity::AttrRole::None,
            )
            .unwrap();
    }
}

#[given(expr = "the GraphLink has an attribute named \"{word}\" of type {word}")]
fn mk_inner_attr(world: &mut MDotWorld, attr: String, attrtype: String) {
    world
        .link
        .inner
        .add_attr(
            attr,
            EntityAttr::try_from(attrtype).unwrap(),
            AttrRole::None,
        )
        .unwrap();
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
        .add_attr(
            world.attr_name.clone(),
            world.attribute.clone(),
            stag::entity::AttrRole::None,
        )
        .unwrap();
}

#[when(expr = "we delete the attribute \"{word}\" from GraphLink")]
fn del_attr(world: &mut MDotWorld, attr: String) {
    world.link.inner.del_attr(attr).unwrap();
}

#[given(expr = "the entity \"{word}\" is known by \"{word}\"")]
#[when(expr = "we add a link on \"{word}\" to \"{word}\"")]
fn add_lk(world: &mut MDotWorld) {
    world.link.link_to(world.entity.clone()).unwrap();
}

#[when(expr = "we remove the link on \"{word}\" to \"{word}\"")]
fn unlink(world: &mut MDotWorld) {
    world.link.unlink(world.entity.clone()).unwrap();
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
        world.link.inner.get_attr(attr).unwrap().clone().0,
        grphlk_attr
    );
}

#[then(expr = "the GraphLink doesn't have an attribute named \"{word}\"")]
fn check_no_attr(world: &mut MDotWorld, attr: String) {
    match world.link.inner.get_attr(attr) {
        Ok(_) => panic!("This is not supposed to happen"),
        Err(_) => { /* this is nice */ }
    }
}

#[then(expr = "the GraphLink does know \"{word}\"")]
fn check_known(world: &mut MDotWorld, entity: String) {
    world.link.get_lk(entity).unwrap();
}

#[then(expr = "the GraphLink does not know \"{word}\"")]
fn check_unknown(world: &mut MDotWorld, entity: String) {
    match world.link.get_lk(entity) {
        Ok(_) => panic!("There should not be an entity"),
        Err(_) => { /* Nothing to see */ }
    }
}
