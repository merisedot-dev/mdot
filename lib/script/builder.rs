use crate::{errors::StagResult, graph::Graph, script::ConversionCore};

pub struct ScriptBuilder {
    name: String,
    conversion_core: Box<dyn ConversionCore + Send + Sync>,
    // TODO necessary conversion info
}

impl ScriptBuilder {
    pub fn new(
        name: impl ToString,
        conversion_core: Box<dyn ConversionCore + Send + Sync>,
    ) -> Self {
        Self {
            name: name.to_string(),
            conversion_core,
        }
    }

    pub fn check(&self, graph: Graph) -> StagResult<Graph> {
        todo!()
    }

    pub fn convert(&self, graph: Graph) -> StagResult<String> {
        todo!()
    }
}
