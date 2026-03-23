use cucumber::given;

use crate::MDotWorld;

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
