mod entity;

use cucumber::World;
use stag::entity::Entity;

#[derive(Debug, Default, World)]
pub struct MDotWorld {
    // Entity testing
    entity: Entity,
    // TODO necessary fields for other test suites
}

fn main() {
    futures::executor::block_on(MDotWorld::run("tests/mdottest/features"));
}
