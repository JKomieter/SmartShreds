mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct RecentsBox(ObjectSubclass<imp::RecentsBox>) 
        @extends gtk::Notebook, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl RecentsBox {
    pub fn new() -> Self {
        Object::builder().build()
    }
}