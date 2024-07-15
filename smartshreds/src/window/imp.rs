
use std::cell::{Cell, RefCell};
use gtk::{self, glib::{self, clone, subclass::InitializingObject}, 
    Button, CompositeTemplate, Label, ListBox, ListBoxRow};
use adw::subclass::prelude::*;
use adw::prelude::*;

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
    pub pagination: TemplateChild<Label>,
    #[template_child]
    pub filesize: TemplateChild<Label>,
    #[template_child]
    pub toastoverlay: TemplateChild<adw::ToastOverlay>,

    #[template_child]
    pub navigationview: TemplateChild<adw::NavigationView>,
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
    // the behaviour of the buttons in the pagination is weird here.
    #[template_callback]
    fn on_next_clicked(&self, _button: &Button) {
        if self.page_number.get() < self.duplicates_vec.borrow().len() && self.duplicates_vec.borrow().len() > 0 {
            self.page_number.set(self.page_number.get() + 1);
            self.obj().present_duplicates();
        }
    }

    #[template_callback]
    fn on_previous_clicked(&self, _button: &Button) {
        if self.page_number.get() > 1 && self.duplicates_vec.borrow().len() > 0 {
            self.page_number.set(self.page_number.get() - 1);
            self.obj().present_duplicates();
        }
    }

    /// Delete the selected duplicate files.
    #[template_callback]
    fn on_delete_clicked(&self, _button: &Button) {
        if self.duplicates_vec.borrow().len() > 0 {
            glib::spawn_future_local(
                clone!(@weak self as window => async move {
                    window.obj().delete_duplicates().await;
                })
            );
        }
    }

    /// The navigation row is clicked.
    #[template_callback]
    fn on_navrow_activated(&self, listboxrow: &ListBoxRow) {
        let index = listboxrow.index();
        // match the index of the row selected to the navigation page tag.
        println!("The index of the row is: {}", index);
        let tag = match index {
            1 => "home",
            2 => "find-duplicates",
            _ => "home",
        };
        self.navigationview.push_by_tag(tag);
    }
}

impl WidgetImpl for SmartShredsWindow {}

impl ApplicationWindowImpl for SmartShredsWindow {}

impl AdwApplicationWindowImpl for SmartShredsWindow {}

impl WindowImpl for SmartShredsWindow {}

