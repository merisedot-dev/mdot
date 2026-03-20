use cucumber::{given, then, when};
use stag::{entity::Cardi, graph::Graph, script::keys::Association};

use crate::{
    MDotWorld,
    utils::{MyBool, str2i8},
};

#[given(expr = "an entity named \"{word}\" in graph")]
fn ensure_entity(world: &mut MDotWorld, name: String) {
    world.graph.mk_entity(name).unwrap();
}

#[given(expr = "the cardinalities on entity 1 will be {int},{word}")]
fn ensure_card1(world: &mut MDotWorld, min: i8, max: String) {
    world.cardis1 = (Cardi::from(min), Cardi::from(str2i8(max)));
}

#[given(expr = "the cardinalities on entity 2 will be {int},{word}")]
fn ensure_card2(world: &mut MDotWorld, min: i8, max: String) {
    world.cardis2 = (Cardi::from(min), Cardi::from(str2i8(max)));
}

#[given(expr = "the cardinalities on entity 3 will be {int},{word}")]
fn ensure_card3(world: &mut MDotWorld, min: i8, max: String) {
    world.cardis3 = (Cardi::from(min), Cardi::from(str2i8(max)));
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

#[when("we link entities 1 and 2 together")]
fn mk_link_bis(world: &mut MDotWorld) {
    world.graph.link("ctest", "e1", "e2").unwrap();
    // tweak cardinalities on e1
    world
        .graph
        .edt_link("ctest")
        .unwrap()
        .set_cardinality("e1", world.cardis1.clone().0, world.cardis1.clone().1)
        .unwrap();
    // tweak cardinalities on e2
    world
        .graph
        .edt_link("ctest")
        .unwrap()
        .set_cardinality("e2", world.cardis2.clone().0, world.cardis2.clone().1)
        .unwrap();
}

#[when("we link entities 1, 2 and 3 together")]
fn mk_ternary_link(world: &mut MDotWorld) {
    mk_link_bis(world);
    let graph = world.graph.clone();
    // adding third entity
    world
        .graph
        .edt_link("ctest")
        .unwrap()
        .link_to(graph.get_entity("e3").unwrap().clone())
        .unwrap();
    world
        .graph
        .edt_link("ctest")
        .unwrap()
        .set_cardinality("e3", world.cardis3.clone().0, world.cardis3.clone().1)
        .unwrap();
}

#[when(expr = "we extract the association info from \"{word}\"")]
fn extract_assoc(world: &mut MDotWorld, name: String) {
    world.assoc = Association::try_from(world.graph.get_lk(name).unwrap().clone()).unwrap();
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

#[then("the association is a one2many association")]
fn check_one2many(world: &mut MDotWorld) {
    match world.assoc.clone() {
        Association::ONE2MANY(name, nlb) => {
            world.key = name;
            world.nlb1 = nlb;
        }
        _ => panic!("association mismatch"),
    }
}

#[then("the association is a one2one association")]
fn check_one2one(world: &mut MDotWorld) {
    match world.assoc.clone() {
        Association::ONE2ONE(nlb1, nlb2) => {
            world.nlb1 = nlb1;
            world.nlb2 = nlb2;
        }
        _ => panic!("association mismatch"),
    }
}

#[then("the association is a many2many association")]
fn check_many2many(world: &mut MDotWorld) {
    match world.assoc.clone() {
        Association::MANY2MANY(name) => {
            world.key = name;
        }
        _ => panic!("association mismatch"),
    }
}

#[then("the association is a ternary association")]
fn check_ternary(world: &mut MDotWorld) {
    match world.assoc.clone() {
        Association::TERNARY(name) => {
            world.key = name;
        }
        _ => panic!("association mismatch"),
    }
}

#[then(expr = "the one2many key is on {int} and is nullable [{word}]")]
fn check_o2m_nullable(world: &mut MDotWorld, ent: usize, bl: MyBool) {
    assert_eq!(world.key, format!("e{}", ent));
    assert_eq!(world.nlb1, bl.into());
}

#[then(expr = "the key on 1 is nullable [{word}]")]
fn check_cardis1(world: &mut MDotWorld, status: MyBool) {
    assert_eq!(world.nlb1, status.into())
}

#[then(expr = "the key on 2 is nullable [{word}]")]
fn check_cardis2(world: &mut MDotWorld, status: MyBool) {
    assert_eq!(world.nlb2, status.into())
}

#[then(expr = "the intermediate's entity name is \"{word}\"")]
fn check_intermediate(world: &mut MDotWorld, name: String) {
    assert_eq!(world.key, name)
}
