
use std::cell::{Cell, RefCell};
use gtk::{self, glib::{self, clone, subclass::InitializingObject}, 
    Button, CompositeTemplate, Label, ListBox, ListBoxRow, Box};
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::types::DupFile;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/SmartShreds/window.ui")]
pub struct SmartShredsWindow {
    // home page
    #[template_child]
    pub file_type_boxes: TemplateChild<Box>,

    // duplicate page.
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
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SmartShredsWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.display_file_types();
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
                clone!(#[weak(rename_to = window)] self, async move {
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
        let tag = match index {
            1 => "home",
            2 => "find-duplicates",
            _ => "home",
        };
        self.navigationview.pop();
        self.navigationview.push_by_tag(tag);
    }

    /// The scan button is clicked.
    #[template_callback]
    async fn on_select_directory_clicked(&self, _button: &Button) {
        self.obj().select_folder_to_scan().await;
    }

}

impl WidgetImpl for SmartShredsWindow {}

impl ApplicationWindowImpl for SmartShredsWindow {}

impl AdwApplicationWindowImpl for SmartShredsWindow {}

impl WindowImpl for SmartShredsWindow {}

