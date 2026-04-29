mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct MDotGraphRow(ObjectSubclass<imp::MDotGraphRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

// creation implementation
impl MDotGraphRow {
    /// Creates a new [MDotGraphRow] depending of its label and the icon name.
    /// We are not using any fancy icons on this one, regular symbolics will do.
    /// Any case of aberrant values isn't the [MDotGraphRow]'s problem.
    pub fn new(label: impl ToString, icon: impl ToString) -> Self {
        Object::builder()
            .property("it_name", label.to_string())
            .property("icon_name", icon.to_string())
            .build()
    }
}
