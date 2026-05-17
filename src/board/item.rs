use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct DrawnItem(ObjectSubclass<imp::DrawnItem>);
}

impl DrawnItem {
    pub fn new(ent: impl ToString, x: impl Into<i32>, y: impl Into<i32>) -> Self {
        Object::builder()
            .property("ent_name", ent.to_string())
            .property("x", x.into())
            .property("y", y.into())
            .build()
    }
}

// implementation holder
mod imp {
    use gtk::glib::{self, Properties, prelude::*, subclass::prelude::*};
    use std::cell::OnceCell;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::DrawnItem)]
    pub struct DrawnItem {
        #[property(get, set)]
        pub ent_name: OnceCell<String>,
        #[property(get, set)]
        pub x: OnceCell<i32>,
        #[property(get, set)]
        pub y: OnceCell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DrawnItem {
        const NAME: &'static str = "DrawnItem";
        type Type = super::DrawnItem;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DrawnItem {}
}
