use std::cell::RefCell;

use gtk::{
    CompositeTemplate, Snapshot,
    gio::{ListStore, prelude::ListModelExtManual},
    glib::{self, subclass::InitializingObject},
    subclass::prelude::*,
};
use tracing::info;

use crate::board::{item::DrawnItem, lines::DrawnLine};

#[derive(CompositeTemplate)]
#[template(resource = "/com/github/merisedotdev/mdot/mdot_drawing.ui")]
pub struct MDotDrawingBoard {
    // information intermediates
    pub items: RefCell<ListStore>,
    pub lines: RefCell<ListStore>,
}

#[glib::object_subclass]
impl ObjectSubclass for MDotDrawingBoard {
    const NAME: &'static str = "MDotDrawingBoard";
    type Type = super::MDotDrawingBoard;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        // Loading template info
        klass.bind_template();
        info!("Loaded MDotDrawingBoard template");
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template(); // build widget from template info
    }
}

// core GObject override
impl ObjectImpl for MDotDrawingBoard {}

// core GtkWidget override
impl WidgetImpl for MDotDrawingBoard {
    fn snapshot(&self, snapshot: &Snapshot) {
        self.parent_snapshot(snapshot);
        // draw entities
        for ent_name in self
            .items
            .borrow()
            .iter::<DrawnItem>()
            .filter_map(|i| match i {
                Ok(val) => Some(val),
                Err(_) => None,
            })
        {
            // TODO fetch root widget
            // TODO fetch graph from window
            // TODO draw entity
            // TODO segment drawing
        }
    }
}

// override for GtkDrawingArea
impl Default for MDotDrawingBoard {
    fn default() -> Self {
        Self {
            items: ListStore::new::<DrawnItem>().into(),
            lines: ListStore::new::<DrawnLine>().into(),
        }
    }
}
