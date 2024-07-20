use adw::subclass::prelude::*;
use gtk::{CompositeTemplate, glib};


#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/SmartShreds/recents_box.ui")]
pub struct RecentsBox {

}

#[glib::object_subclass]
impl ObjectSubclass for RecentsBox {
    const NAME: &'static str = "RecentsBox";
    type ParentType = gtk::Box;
    type Type = super::RecentsBox;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for RecentsBox {}

impl WidgetImpl for RecentsBox {}

impl BoxImpl for RecentsBox {}
