use cucumber::given;
use rand::seq::SliceRandom;

use crate::MDotWorld;

#[given(expr = "there are {int} entities in graph")]
fn ensure_nb_entities(world: &mut MDotWorld, nb: usize) {
    for i in 0..nb {
        world.graph.mk_entity(format!("e{}", i)).unwrap();
    }
}

#[given(expr = "there are {int} links in graph")]
fn random_lks(world: &mut MDotWorld, nb: usize) {
    let mut rng = rand::rng();
    let mut nums: Vec<usize> = (0..world.graph.get_entities().len()).collect();
    for i in 0..nb {
        nums.shuffle(&mut rng);
        world
            .graph
            .link(
                format!("lk{}", i),
                format!("e{}", nums[0]),
                format!("e{}", nums[1]),
            )
            .unwrap();
    }
}

#[given(expr = "the entity {int} is linked with {int} via {word}")]
fn mk_lk(world: &mut MDotWorld, e1: usize, e2: usize, lk: String) {
    let graph = world.graph.clone();
    match graph.get_lk(lk.clone()) {
        Ok(_) => world.graph.extra_lk(lk, format!("e{}", e2)).unwrap(),
        _ => world
            .graph
            .link(lk, format!("e{}", e1), format!("e{}", e2))
            .unwrap(),
    }
}

#[given(expr = "we want to name the database \"{word}\"")]
fn mk_name(world: &mut MDotWorld, name: String) {
    world.name = name;
}
