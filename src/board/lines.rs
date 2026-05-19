use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct DrawnLine(ObjectSubclass<imp::DrawnLine>);
}

impl DrawnLine {
    /// Builds a new [DrawnLine] from point coordinates. These may not match what
    /// is on screen at any given time.
    ///
    /// **Warning**: If `a` and `b` are identical, the line won't be built.
    pub fn new(a: (i32, i32), b: (i32, i32)) -> Option<Self> {
        if a == b {
            // identical points
            return None;
        }
        // building the line
        Some(
            Object::builder()
                .property("x_start", a.0)
                .property("y_start", a.1)
                .property("x_end", b.0)
                .property("x_end", b.1)
                .build(),
        )
    }

    /// Fetches the starting point of the [DrawnLine].
    pub fn start(&self) -> (i32, i32) {
        (self.x_start().into(), self.y_start().into())
    }

    /// Fetches the end point of the [DrawnLine].
    pub fn end(&self) -> (i32, i32) {
        (self.x_end().into(), self.y_end().into())
    }
}

mod imp {
    use gtk::glib::{self, Properties, prelude::*, subclass::prelude::*};
    use std::cell::OnceCell;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::DrawnLine)]
    pub struct DrawnLine {
        // starting point
        #[property(get, set)]
        pub x_start: OnceCell<i32>,
        #[property(get, set)]
        pub y_start: OnceCell<i32>,
        // end point
        #[property(get, set)]
        pub x_end: OnceCell<i32>,
        #[property(get, set)]
        pub y_end: OnceCell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DrawnLine {
        const NAME: &'static str = "DrawnLine";
        type Type = super::DrawnLine;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DrawnLine {}
}
