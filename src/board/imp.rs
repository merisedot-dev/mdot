use std::cell::RefCell;

use gtk::{
    CompositeTemplate,
    gio::{ListStore, prelude::*},
    glib::{self, Properties, clone, subclass::InitializingObject},
    prelude::DrawingAreaExtManual,
    subclass::prelude::*,
};
use tracing::info;

use crate::{
    board::{item::DrawnItem, lines::DrawnLine},
    window::project::Project,
};

#[derive(CompositeTemplate, Properties)]
#[template(resource = "/com/github/merisedotdev/mdot/mdot_drawing.ui")]
#[properties(wrapper_type=super::MDotDrawingBoard)]
pub struct MDotDrawingBoard {
    // information intermediates
    pub items: RefCell<ListStore>,
    pub lines: RefCell<ListStore>,
    // binding properties
    #[property(get, set)]
    pub project: RefCell<Project>,
}

#[glib::object_subclass]
impl ObjectSubclass for MDotDrawingBoard {
    const NAME: &'static str = "MDotDrawingBoard";
    type Type = super::MDotDrawingBoard;
    type ParentType = gtk::DrawingArea;

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
#[glib::derived_properties]
impl ObjectImpl for MDotDrawingBoard {
    fn constructed(&self) {
        // super() call
        self.parent_constructed();
        // inner setup
        DrawingAreaExtManual::set_draw_func(
            self.obj().as_ref(),
            clone!(
                #[weak(rename_to=widget)]
                self,
                move |_, ctx, _, _| {
                    ctx.set_source_rgba(50., 0., 0., 0.);
                    let _ = ctx.fill();
                    // TODO test scenario
                    // TODO prefetch graph
                    // TODO draw a single entity
                }
            ),
        )
    }
}

// override GtkDrawingArea
impl DrawingAreaImpl for MDotDrawingBoard {}

// core GtkWidget override
impl WidgetImpl for MDotDrawingBoard {}

// override for GtkDrawingArea
impl Default for MDotDrawingBoard {
    fn default() -> Self {
        Self {
            items: ListStore::new::<DrawnItem>().into(),
            lines: ListStore::new::<DrawnLine>().into(),
            project: RefCell::new(Project::default()),
        }
    }
}
