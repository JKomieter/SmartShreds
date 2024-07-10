mod imp;

use std::path::PathBuf;

use gtk::{self, glib::{self, Object}};

glib::wrapper! {
    pub struct DupFileObject(ObjectSubclass<imp::DupFileObject>);
}

impl DupFileObject {
    pub fn new(
        filename: String,
        filesize: String,
        filepath: PathBuf,
    ) -> Self {
        Object::builder()
            .property("filename", filename)
            .property("filesize", filesize)
            .property("filepath", filepath)
            .build()
    }
}

#[derive(Default)]
pub struct DupFileData {
    pub filename: String,
    pub filesize: String,
    pub filepath: PathBuf,
}