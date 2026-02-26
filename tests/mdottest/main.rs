mod entity;
mod link;

use cucumber::World;
use stag::entity::{Entity, EntityAttr, GraphLink};

#[derive(Debug, Default, World)]
pub struct MDotWorld {
    // Entity testing
    entity: Entity,
    attribute: EntityAttr,
    attr_name: String,
    // GraphLink testing
    link: GraphLink,
    // TODO necessary fields for other test suites
}

fn main() {
    futures::executor::block_on(MDotWorld::run("tests/mdottest/features"));
}
