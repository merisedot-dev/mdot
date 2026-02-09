use crate::{
    utils::{MDotActable, MDotDialog},
    window::Window,
};

pub struct ScriptDialog;

impl MDotActable for ScriptDialog {
    type InnerCallerType = Window;
}

impl MDotDialog for ScriptDialog {
    async fn handle_dialog(&self, _: Self::InnerCallerType) {
        // TODO
    }
}
