mod imp;

use gtk::glib;

glib::wrapper! {pub struct Project(ObjectSubclass<imp::MDotProject>);}
