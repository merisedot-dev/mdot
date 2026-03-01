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
            .link(format!("lk{}", i), format!("e"), format!("e"))
            .unwrap();
    }
}
