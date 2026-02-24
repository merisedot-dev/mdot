use gtk::glib::Variant;

use crate::utils::MDotActable;

/// Encapsulation trait, this will define how a GTK-related action will behave
/// here. Please keep in mind that an action may not be suited to be ported over
/// another subclassed widget that it was made for.
pub trait MDotAction: MDotActable {
    /// The action's name. Please keep the ENTIRE name in there.
    fn name(&self) -> &'static str;

    /// activation handler for the action
    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        text: &str,
        variant: Option<&Variant>,
    );
}
