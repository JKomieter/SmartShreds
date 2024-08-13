use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::Duplicate;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::DuplicateObject)]
pub struct DuplicateObject {
    #[property(name = "name", get, set, type = String, member = name)]
    #[property(name = "check", get, set, type = bool, member = check)]
    #[property(name = "size", get, set, type = String, member = size)]
    #[property(name = "path", get, set, type = String, member = path)]
    #[property(name = "date-created", get, set, type = String, member = date_created)]
    #[property(name = "background-color", get, set, type = String, member = background_color)]
    pub data: RefCell<Duplicate>,
}

#[glib::object_subclass]
impl ObjectSubclass for DuplicateObject {
    const NAME: &'static str = "DuplicateObject";
    type Type = super::DuplicateObject;
}

#[glib::derived_properties]
impl ObjectImpl for DuplicateObject {}
