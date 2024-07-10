use std::cell::RefCell;
use std::path::PathBuf;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::DupFileData;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::DupFileObject)]
pub struct DupFileObject {
    #[property(name = "filename", get, set, type = String, member = filename)]
    #[property(name = "filesize", get, set, type = String, member = filesize)]
    #[property(name = "filepath", get, set, type = PathBuf, member = filepath)]
    pub data: RefCell<DupFileData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for DupFileObject {
    const NAME: &'static str = "DupFileObject";
    type Type = super::DupFileObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for DupFileObject {}