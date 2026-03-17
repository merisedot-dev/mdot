mod entity;
mod graph;
mod link;
mod script;
pub(crate) mod utils;

use cucumber::World;
use stag::{
    entity::{AttrRole, Cardi, Entity, EntityAttr, GraphLink},
    graph::Graph,
    script::keys::Association,
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
    cards: Vec<(Cardi, Cardi)>,
    assoc: Association,
}

#[tokio::main]
async fn main() {
    MDotWorld::cucumber()
        .init_tracing()
        .run("tests/mdottest/features")
        .await;
}
