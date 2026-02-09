use adw::Window;

use crate::utils::MDotDialog;

mod pick_proj;
mod script;

pub(super) fn mk_dialogs() -> Vec<Box<dyn MDotDialog<InnerCallerType = Window>>> {
    vec![
        Box::new(pick_proj::PickProjDialog),
        Box::new(script::ScriptDialog),
    ]
}
