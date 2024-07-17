use adw::subclass::prelude::*;
use gtk::{CompositeTemplate, Image, Box, glib, Label, ProgressBar};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/SmartShreds/file_type_box.ui")]
pub struct FileTypeBox {
    #[template_child]
    pub card_icon: TemplateChild<Image>,
    #[template_child]
    pub file_type_label: TemplateChild<Label>,
    #[template_child]
    pub file_count_label: TemplateChild<Label>,
    #[template_child]
    pub size_label: TemplateChild<Label>,
    #[template_child]
    pub size_progress: TemplateChild<ProgressBar>,
}

#[glib::object_subclass]
impl ObjectSubclass for FileTypeBox {
    const NAME: &'static str = "FileTypeBox";
    type Type = super::FileTypeBox;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for FileTypeBox {}

impl ObjectImpl for FileTypeBox {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for FileTypeBox {}
