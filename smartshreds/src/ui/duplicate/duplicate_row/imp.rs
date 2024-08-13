use std::cell::RefCell;

use adw::subclass::prelude::*;
use gtk::{
    self, glib::{self, Binding}, Box, CheckButton, CompositeTemplate, Label
};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/SmartShreds/duplicate_row.ui")]
pub struct DuplicateRow {
    #[template_child]
    pub check: TemplateChild<CheckButton>,
    #[template_child]
    pub name: TemplateChild<Label>,
    #[template_child]
    pub size: TemplateChild<Label>,
    #[template_child]
    pub date_created: TemplateChild<Label>,
    #[template_child]
    pub path: TemplateChild<Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for DuplicateRow {
    const NAME: &'static str = "DuplicateRow";
    type Type = super::DuplicateRow;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for DuplicateRow {}

impl ObjectImpl for DuplicateRow {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for DuplicateRow {}