
use std::cell::{Cell, RefCell};
use gtk::{self, glib::{self, subclass::InitializingObject}, Button, CompositeTemplate, ListBox };
use adw::subclass::prelude::*;

use crate::types::DupFile;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/SmartShreds/window.ui")]
pub struct SmartShredsWindow {
    // First page showing he duplicate files.
    #[template_child]
    pub listbox: TemplateChild<ListBox>,
    pub duplicates_vec: RefCell<Vec<Vec<DupFile>>>,
    pub page_number: Cell<usize>,
    #[template_child]
    pub pagination: TemplateChild<Button>
}

#[glib::object_subclass]
impl ObjectSubclass for SmartShredsWindow {
    const NAME: &'static str = "SmartShredsWindow";
    type ParentType = adw::ApplicationWindow;
    type Type = super::SmartShredsWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
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

#[gtk::template_callbacks]
impl SmartShredsWindow {
    #[template_callback]
    fn handle_next_button_clicked(&self, _button: &Button) {
        if self.page_number.get() > self.duplicates_vec.borrow().len() {
            return;
        }
        self.page_number.set(self.page_number.get() + 1);
        self.obj().present_duplicates();
    }

    #[template_callback]
    fn handle_prev_button_clicked(&self, _button: &Button) {
        if self.page_number.get() == 1 {
            return;
        }
        self.page_number.set(self.page_number.get() - 1);
        self.obj().present_duplicates();
    
    }
}

impl WidgetImpl for SmartShredsWindow {}

impl ApplicationWindowImpl for SmartShredsWindow {}

impl AdwApplicationWindowImpl for SmartShredsWindow {}

impl WindowImpl for SmartShredsWindow {}

