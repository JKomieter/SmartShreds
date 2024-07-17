mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct FileTypeBox(ObjectSubclass<imp::FileTypeBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl FileTypeBox {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
