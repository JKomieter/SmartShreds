use std::cell::RefCell;

use gtk::{self, glib::{self, Binding}, CompositeTemplate, Label};
use adw::subclass::prelude::*;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/SmartShreds/dup_file_row.ui")]
pub struct DupFileRow {
    #[template_child]
    pub filename: TemplateChild<Label>,
    #[template_child]
    pub filesize: TemplateChild<Label>,
    #[template_child]
    pub filepath: TemplateChild<Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for DupFileRow {
    const NAME: &'static str = "DupFileRow";
    type ParentType = gtk::ListBoxRow;
    type Type = super::DupFileRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for DupFileRow {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for DupFileRow {}

impl ListBoxRowImpl for DupFileRow {}