mod entity;

use cucumber::World;
use stag::entity::Entity;

#[derive(Debug, Default, World)]
pub struct MDotWorld {
    entity: Entity,
}

fn main() {
    futures::executor::block_on(MDotWorld::run("tests/mdottest/features"));
}
