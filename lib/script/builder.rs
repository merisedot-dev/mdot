use std::collections::HashMap;

use crate::{
    constants::FOOTER,
    errors::StagResult,
    script::{ConversionCore, overlay::imp::GraphOverlay},
};

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

    /// Turns a given subgraph type [HashMap] into a [Vec] of SQL-compliant
    /// [String]s. This is meant to be an intermediate in calculus, not an
    /// exposed function.
    fn turn<T, F>(&self, params: HashMap<String, T>, f: F) -> Vec<String>
    where
        F: FnMut((&String, &T)) -> StagResult<String>,
    {
        params
            .iter()
            .map(f)
            .collect::<Vec<StagResult<String>>>()
            .iter()
            .filter(|i| match i {
                Ok(_) => true,
                Err(_) => false,
            })
            .map(|st| st.as_ref().unwrap().clone())
            .collect::<Vec<String>>()
    }

    /// Turns the given [GraphOverlay] into a functional SQL script, ready to be
    /// written in a file.
    pub fn convert(&self, graph: GraphOverlay) -> StagResult<String> {
        let mut ggraph = graph.clone();
        ggraph.check()?;
        tracing::info!("blep");
        Ok(format!(
            "{}\n\n{}",
            self.conversion_core.header(self.name.clone()),
            // paperwork
            FOOTER
        ))
    }
}
