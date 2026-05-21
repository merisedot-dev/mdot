mod imp;
mod item;
mod lines;

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio::prelude::ListModelExtManual, glib};

use crate::board::item::DrawnItem;

glib::wrapper! {
    pub struct MDotDrawingBoard(ObjectSubclass<imp::MDotDrawingBoard>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Root;
}

// logic implemenation
impl MDotDrawingBoard {
    /// Store a new [DrawnItem] to the [MDotDrawingBoard]'s store. This will not
    /// update any existing drawable with the same name. Negative coordinates are
    /// allowed for the [DrawnItem].
    pub fn add_drawitem(&self, name: impl ToString, x: i32, y: i32) {
        let store = self.imp().items.borrow();
        let str_name = name.to_string();
        if let None = store
            .iter::<DrawnItem>()
            .filter(|i| match i {
                Ok(val) => val.ent_name() == str_name,
                Err(_) => false,
            })
            .last()
        {
            // the item doesn't already exist
            store.append(&DrawnItem::new(str_name, x, y));
        }
    }
}
