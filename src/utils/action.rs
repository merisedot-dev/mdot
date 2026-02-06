use gtk::glib::Variant;

/// Encapsulation trait, this will define how a GTK-related action will behave
/// here. Please keep in mind that an action may not be suited to be ported over
/// another subclassed widget that it was made for.
pub trait MDotAction {
    /// The action's name. Please remove the first part of said name and keep
    /// only the last one.
    const NAME: &'static str;

    /// The type of the thing that will use the action. Said type must implement
    /// the [ObjectSubclass] trait.
    type InnerCallerType; // TODO ensure this is an ObjectSubclass implementor.

    /// activation handler for the action
    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        text: &str,
        variant: Option<&Variant>,
    );
}
