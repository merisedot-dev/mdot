mod entity;
mod graph;
mod link;
mod script;

use cucumber::World;
use stag::{
    entity::{AttrRole, Entity, EntityAttr, GraphLink},
    graph::Graph,
};

#[derive(Debug, Default, World)]
pub struct MDotWorld {
    // Entity testing
    entity: Entity,
    attribute: EntityAttr,
    attr_name: String,
    role: AttrRole,
    // GraphLink testing
    link: GraphLink,
    // Graph testing
    graph: Graph,
    name: String,
    // TODO necessary fields for other test suites
}

fn main() {
    futures::executor::block_on(MDotWorld::run("tests/mdottest/features"));
}
