mod imp;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib::{self, Object};

use super::duplicate_object::DuplicateObject;

glib::wrapper! {
    pub struct DuplicateRow(ObjectSubclass<imp::DuplicateRow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl DuplicateRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, duplicate_object: &DuplicateObject) {
        let name = self.imp().name.get();
        let check = self.imp().check.get();
        let size = self.imp().size.get();
        let path = self.imp().path.get();
        let date_created = self.imp().date_created.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        let check_binding = duplicate_object
            .bind_property("check", &check, "active")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(check_binding);

        let name_binding = duplicate_object
            .bind_property("name", &name, "label")
            .sync_create()
            .build();
        bindings.push(name_binding);

        let size_binding = duplicate_object
            .bind_property("size", &size, "label")
            .sync_create()
            .build();
        bindings.push(size_binding);

        let path_binding = duplicate_object
            .bind_property("path", &path, "label")
            .sync_create()
            .build();
        bindings.push(path_binding);

        let date_created_binding = duplicate_object
            .bind_property("date-created", &date_created, "label")
            .sync_create()
            .build();
        bindings.push(date_created_binding);

        self.set_property("name", duplicate_object.background_color().as_str());
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
