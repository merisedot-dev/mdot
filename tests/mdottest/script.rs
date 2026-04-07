use std::{fs::{File, read}, io::Read};

use cucumber::{given, then, when};
use stag::{
    entity::Cardi,
    script::{MySQLCore, ScriptBuilder, imp::GraphOverlay},
};

use crate::{
    MDotWorld,
    utils::{parseu8, str2i8},
};

#[given(expr = "there are {int} entities in graph")]
fn ensure_nb_entities(world: &mut MDotWorld, nb: usize) {
    for i in 1..nb + 1 {
        world.graph.mk_entity(format!("e{}", i)).unwrap();
    }
}

#[given("each entity has a primary key")]
fn ensure_pk(world: &mut MDotWorld) {
    let _graph = world.graph.clone();
}

#[given(expr = "we want to name the database \"{word}\"")]
fn ensure_dbname(world: &mut MDotWorld, name: String) {
    world.name = name;
}

#[given(expr = "entities {int} and {int} are linked via \"{word}\"")]
#[given(expr = "the entity {int} is linked with {int} via \"{word}\"")]
fn ensure_entlk(world: &mut MDotWorld, e1: i8, e2: String, name: String) {
    world
        .graph
        .link(name.clone(), format!("e{}", e1), format!("e{}", e2))
        .unwrap();
    world.lk_name = name;
}

#[given(expr = "ent{int}'s cardinalities are {int},{word}")]
fn ensure_card(world: &mut MDotWorld, e: i8, n: i8, m: String) {
    let lk = world.graph.edt_link(world.lk_name.clone()).unwrap();
    lk.set_cardinality(format!("e{}", e), Cardi::from(n), Cardi::from(str2i8(m)))
        .unwrap();
}

#[when("we convert the graph using the MySql conversion core")]
fn convert(world: &mut MDotWorld) {
    world.script = ScriptBuilder::new(world.name.clone(), Box::new(MySQLCore))
        .convert(GraphOverlay::new(world.graph.clone()))
        .unwrap();
}

#[then(expr = "the resulting script looks like `{word}`")]
async fn check_script(world: &mut MDotWorld, path: String) {
    let mut file = File::open(format!("tests/mdottest/{}", path)).unwrap();
    // read script contents
    let mut contents: Vec<u8> = vec![];
    file.read_to_end(&mut contents).unwrap();
    // assertion
    assert_eq!(world.script, parseu8(contents));
}
