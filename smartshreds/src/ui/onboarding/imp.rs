use adw::subclass::prelude::*;
use gtk::{CompositeTemplate, glib};


#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/SmartShreds/onboarding.ui")]
pub struct OnBoarding {
    #[template_child]
    pub navigation_view: TemplateChild<adw::NavigationView>,
    #[template_child]
    pub role: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub feature: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub recv_notifications: TemplateChild<gtk::Switch>
}

#[glib::object_subclass]
impl ObjectSubclass for OnBoarding {
    const NAME: &'static str = "OnBoarding";
    type ParentType = gtk::Box;
    type Type = super::OnBoarding;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for OnBoarding {}

impl WidgetImpl for OnBoarding {}

impl BoxImpl for OnBoarding {}
