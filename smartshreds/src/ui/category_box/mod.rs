mod imp;

use gtk::glib::{self, Object};
use gtk;

glib::wrapper! {
    pub struct CategoryBox(ObjectSubclass<imp::CategoryBox>) 
        @extends gtk::FlowBoxChild, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CategoryBox {
    pub fn new() -> Self {
        Object::builder().build()
    }
}