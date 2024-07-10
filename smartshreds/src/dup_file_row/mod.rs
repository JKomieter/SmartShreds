mod imp;

use adw::prelude::*;
use gtk::{self, glib::{self, Object}};
use adw::subclass::prelude::*;

use crate::dup_file_object::DupFileObject;


glib::wrapper! {
    pub struct DupFileRow(ObjectSubclass<imp::DupFileRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for DupFileRow {
    fn default() -> Self {
        Self::new()
    }
}

impl DupFileRow {
    pub fn new() -> Self {
        Object::builder().build()
    } 

    pub fn bind(&self, dup_file_object: &DupFileObject) {
        let file_name = self.imp().filename.get();
        let file_size = self.imp().filesize.get();
        let file_path = self.imp().filepath.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // bind the file_name to the DupFileObject's file_name property
        let file_name_binding = dup_file_object
            .bind_property("filename", &file_name, "label")
            .bidirectional()
            .sync_create()
            .build();
        let file_size_binding = dup_file_object
            .bind_property("filesize", &file_size, "label")
            .bidirectional()
            .sync_create()
            .build();
        let file_path_binding = dup_file_object
            .bind_property("filepath", &file_path, "label")
            .bidirectional()
            .sync_create()
            .build();

        bindings.push(file_name_binding);
        bindings.push(file_size_binding);
        bindings.push(file_path_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}