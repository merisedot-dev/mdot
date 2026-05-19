use std::cell::RefCell;

use gtk::{
    CompositeTemplate, Snapshot,
    gio::ListStore,
    glib::{self, subclass::InitializingObject},
    gsk::Stroke,
    subclass::prelude::*,
};
use tracing::info;

use crate::board::item::DrawnItem;

#[derive(CompositeTemplate)]
#[template(resource = "/com/github/merisedotdev/mdot/mdot_drawing.ui")]
pub struct MDotDrawingBoard {
    // information intermediates
    pub items: RefCell<ListStore>,
    // TODO add link positions definition
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
        let stroke = Stroke::new(2.);
        // TODO draw stuff
        // TODO store drawn stuff
    }
}

// override for GtkDrawingArea
impl Default for MDotDrawingBoard {
    fn default() -> Self {
        Self {
            items: ListStore::new::<DrawnItem>().into(),
        }
    }
}
