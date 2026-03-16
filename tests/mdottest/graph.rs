use cucumber::{given, then, when};
use stag::{entity::Cardi, graph::Graph, script::keys::Association};

use crate::{MDotWorld, utils::str2assoc};

#[given(expr = "an entity named \"{word}\" in graph")]
fn ensure_entity(world: &mut MDotWorld, name: String) {
    world.graph.mk_entity(name).unwrap();
}

#[given(expr = "the cardinalities on entity {int} will be {int},{word}")]
fn ensure_card(world: &mut MDotWorld, _name: usize, n: i8, m: String) {
    world.cards.push((
        Cardi::from(n),
        Cardi::from(if m == "n" {
            -1
        } else {
            m.as_str().parse::<i8>().unwrap()
        }),
    ));
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
    world.graph.extra_lk(glk, entity).unwrap();
}

#[when(expr = "we link entities {int} and {int} together")]
fn mk_speclink(world: &mut MDotWorld, e1: usize, e2: usize) {
    world
        .graph
        .link("ctest", format!("e{}", e1), format!("e{}", e2))
        .unwrap();
    // setting cardinalities on 1
    world
        .graph
        .edt_link("ctest")
        .unwrap()
        .set_cardinality(
            "e1",
            world.cards[e1 - 1].0.clone(),
            world.cards[e1 - 1].1.clone(),
        )
        .unwrap();
    // setting cardinalities on 2
    world
        .graph
        .edt_link("ctest")
        .unwrap()
        .set_cardinality(
            "e2",
            world.cards[e2 - 1].0.clone(),
            world.cards[e2 - 1].1.clone(),
        )
        .unwrap();
}

#[when(expr = "we extract the association info from \"{word}\"")]
fn extract_assoc(world: &mut MDotWorld, name: String) {
    world.assoc = world
        .graph
        .get_lk(name)
        .unwrap()
        .clone()
        .try_into()
        .unwrap();
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
    world
        .graph
        .get_lk(lk)
        .unwrap()
        .get_entity_link(ent)
        .unwrap();
}

#[then(expr = "the cardinality for entity {int} is {int},{word}")]
fn check_card(world: &mut MDotWorld, name: usize, n: i8, m: String) {
    match world
        .graph
        .get_lk("ctest")
        .unwrap()
        .get_entity_link(format!("e{}", name))
    {
        Ok((_, cn, cm)) => assert!(
            cn.clone() == Cardi::from(n)
                && cm.clone()
                    == Cardi::from(if m == "n" {
                        -1
                    } else {
                        m.as_str().parse::<i8>().unwrap()
                    })
        ),
        _ => panic!("There should be a GraphLink here"),
    }
}

#[then(expr = "the association is of type {word}")]
fn check_assoc(world: &mut MDotWorld, asc: String) {
    match (world.assoc.clone(), str2assoc(asc)) {
        (Association::MANY2MANY, Association::MANY2MANY)
        | (Association::ONE2MANY(_), Association::ONE2MANY(_))
        | (Association::ONE2ONE(_), Association::ONE2ONE(_)) => { /* Nothing */ }
        (_, _) => panic!("This is not a match"),
    }
}
