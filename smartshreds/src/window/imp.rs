
use gtk::{self, glib::{self, subclass::InitializingObject}, Button, CompositeTemplate, };
use adw::subclass::prelude::*;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/SmartShreds/window.ui")]
pub struct SmartShredsWindow {
    #[template_child]
    pub dir_button: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for SmartShredsWindow {
    const NAME: &'static str = "SmartShredsWindow";
    type ParentType = adw::ApplicationWindow;
    type Type = super::SmartShredsWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        // Action to scan a chosen directory.
        klass.install_action_async("start-scan", None, |window, _, _| async move {
            window.select_folder_to_scan().await;
        });
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SmartShredsWindow {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for SmartShredsWindow {}

impl ApplicationWindowImpl for SmartShredsWindow {}

impl AdwApplicationWindowImpl for SmartShredsWindow {}

impl WindowImpl for SmartShredsWindow {}

