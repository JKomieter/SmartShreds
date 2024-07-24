use adw::subclass::prelude::*;
use gtk::{
    self,
    glib::{self, subclass::InitializingObject},
    CompositeTemplate
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/SmartShreds/category_box.ui")]
pub struct CategoryBox {

}

#[glib::object_subclass]
impl ObjectSubclass for CategoryBox {
    const NAME: &'static str = "CategoryBox";
    type Type = super::CategoryBox;
    type ParentType = gtk::FlowBoxChild;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}


impl ObjectImpl for CategoryBox {}

impl WidgetImpl for CategoryBox {}

impl FlowBoxChildImpl for CategoryBox {}
