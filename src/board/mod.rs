mod imp;
mod item;

use gtk::glib;

glib::wrapper! {
    pub struct MDotDrawingBoard(ObjectSubclass<imp::MDotDrawingBoard>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Root;
}
