mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct DuplicateObject(ObjectSubclass<imp::DuplicateObject>);
}

impl DuplicateObject {
    pub fn new(name: String, size: String, path: String, date_created: String, bgcolor: String) -> Self {
        Object::builder()
            .property("name", &name)
            .property("size", &size)
            .property("path", &path)
            .property("date-created", &date_created)
            .property("check", &false)
            .property("background-color", &bgcolor)
            .build()
    }
}

#[derive(Default, Clone, Debug)]
pub struct Duplicate {
    pub name: String,
    pub check: bool,
    pub size: String,
    pub path: String,
    pub date_created: String,
    pub background_color: String,
}
