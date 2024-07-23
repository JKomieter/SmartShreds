mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct OnBoarding(ObjectSubclass<imp::OnBoarding>) 
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl OnBoarding {
    pub fn new() -> Self {
        Object::builder().build()
    }
}