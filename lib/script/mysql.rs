use crate::{
    entity::EntityAttr,
    errors::StagResult,
    graph::Graph,
    script::{ConversionCore, builder::ScriptBuilder},
};

pub struct MySqlConversionCore;

impl ConversionCore for MySqlConversionCore {
    fn convert(&self, _: Graph) -> StagResult<String> {
        let script = ScriptBuilder::new("");
        // TODO conversion
        Ok(script.to_string())
    }

    fn check_type(&self, attr: EntityAttr) -> StagResult<EntityAttr> {
        match attr {
            // TODO add problematic cases
            _ => Ok(attr),
        }
    }
}
