mod imp;

use adw::subclass::prelude::*;
use adw::{prelude::*, AlertDialog, ResponseAppearance, Toast};
use glib::clone;
use glib::Object;
use gtk::gio::{Cancellable, Settings};
use gtk::{gio, glib, FileDialog, Label, ListBoxRow, Spinner};
use std::path::PathBuf;

use crate::types::{AuthResponse, AuthSettings, Category, DupFile};
use crate::ui::category_box::CategoryBox;
use crate::ui::dup_file_object::DupFileObject;
use crate::ui::dup_file_row::DupFileRow;
use crate::ui::file_type_box::FileTypeBox;
use crate::ui::recents_box::RecentsBox;
use crate::utils::analysis::StorageAnalysis;
use crate::utils::{
    format_number, format_size, row_tooltip_markup, traverse_directory_for_duplicates,
};

const APP_ID: &str = "org.gtk_rs.SmartShreds";

glib::wrapper! {
    pub struct SmartShredsWindow(ObjectSubclass<imp::SmartShredsWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl SmartShredsWindow {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup(&self) {
        self.display_filetype_analysis();
        self.setup_categories();
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    /// Check if the user is authenticated.
    fn check_authentication(&self) {
        let auth_settings = self.get_auth_settings();
        // if the user is not authenticated, show the onboarding dialog
        if auth_settings.is_authenticated {
        } else {
            self.unauthenticated();
        }
    }

    /// show the onboarding dialog
    fn unauthenticated(&self) {
        self.imp().stack.set_visible_child_name("auth");
    }

    /// get auth settings
    fn get_auth_settings(&self) -> AuthSettings {
        AuthSettings {
            token: self.settings().string("token").to_string(),
            username: self.settings().string("username").to_string(),
            email: self.settings().string("email").to_string(),
            is_authenticated: self.settings().boolean("is-authenticated"),
            first_time: self.settings().boolean("first-time"),
            user_id: self.settings().string("user-id").to_string(),
        }
    }

    /// Open the dialog box to show the scanning is in progress.
    async fn select_folder_to_scan(&self) {
        let file_dialog = FileDialog::builder()
            .title("Choose directory to scan")
            .build();
        file_dialog.select_folder(
            Some(self),
            Some(&Cancellable::new()),
            clone!(
                #[weak(rename_to = window)]
                self,
                move |folder| {
                    if let Ok(folder_name) = folder {
                        let path = folder_name.path().expect("No path found");
                        glib::spawn_future_local(clone!(
                            #[weak]
                            window,
                            async move {
                                window.scan_progress(&path).await;
                            }
                        ));
                    }
                }
            ),
        );
    }

    fn get_duplicates(&self) -> Vec<Vec<DupFile>> {
        self.imp().duplicates_vec.borrow().clone()
    }

    /// Show alert dialog waiting for the scan to complete.
    async fn scan_progress(&self, path: &PathBuf) {
        let spinner = Spinner::builder()
            .spinning(false)
            .tooltip_text("Scanning...")
            .build();

        let cancel_response = "cancel";
        let start_response = "start";
        let dialog = AlertDialog::builder()
            .heading("Press start to scan")
            .extra_child(&spinner)
            .close_response(cancel_response)
            .default_response(start_response)
            .build();
        dialog.add_responses(&[(cancel_response, "Cancel"), (start_response, "Start")]);
        dialog.set_response_appearance(&cancel_response, ResponseAppearance::Destructive);
        dialog.set_response_appearance(&start_response, ResponseAppearance::Suggested);

        // get the response from the dialog
        let response = dialog.choose_future(self).await;
        if response == cancel_response {
            return;
        } else if response == start_response {
            spinner.start();
            let duplicates_map = traverse_directory_for_duplicates(path.to_path_buf());
            // put a check to see if there are any duplicates
            self.imp().duplicates_vec.replace(
                duplicates_map
                    .values()
                    .filter(|files| files.len() > 1)
                    .map(|files| files.clone())
                    .collect(),
            );
            self.imp().page_number.set(1);
            self.present_duplicates();
        }
    }

    /// Present the duplicate files in the listbox.
    fn present_duplicates(&self) {
        self.imp().listbox.remove_all();
        let page_number =
            if self.imp().page_number.get() == self.imp().duplicates_vec.borrow().len() {
                self.imp().page_number.get()
            } else {
                self.imp().page_number.get() % self.imp().duplicates_vec.borrow().len()
            };
        println!("The current page number is: {}", page_number);
        let duplicate_vec = &self.get_duplicates()[page_number - 1];
        self.imp().pagination.set_label(&format!(
            "{} of {}",
            page_number,
            self.imp().duplicates_vec.borrow().len()
        ));

        duplicate_vec.iter().for_each(|dup_file| {
            let row = self.create_dup_row(dup_file);
            let tooltip_markup = row_tooltip_markup(&dup_file.file_path.to_string_lossy());
            row.set_tooltip_text(Some(&tooltip_markup));
            self.imp().listbox.append(&row);
        });
        if duplicate_vec.len() > 0 {
            let filesize = &format!("FILE SIZE: {}", format_size(duplicate_vec[0].file_size));
            self.imp().filesize.set_label(filesize);
        }
    }

    /// Create a listbox row for the duplicate
    // #[inline]
    fn create_dup_row(&self, dup_file: &DupFile) -> ListBoxRow {
        let filesize = format_size(dup_file.file_size);
        let dup_file_object = DupFileObject::new(
            dup_file.file_name.clone(),
            filesize,
            dup_file.file_path.clone(),
        );

        let dup_file_row = DupFileRow::new();
        dup_file_row.bind(&dup_file_object);
        dup_file_row.upcast()
    }

    /// Delete the selected duplicate files.
    /// ⛔️ This function is dangerous as it completely deletes files from the disk. ⛔️
    async fn delete_duplicates(&self) {
        let delete_response = "delete";
        let cancel_response = "cancel";
        let message = Label::builder()
            .label("Are you sure you want to delete the selected files?")
            .css_classes(*&["warning", "title-2"])
            .build();
        let dialog = AlertDialog::builder()
            .heading("Confirm deletion")
            .extra_child(&message)
            .close_response(delete_response)
            .default_response(cancel_response)
            .build();
        dialog.add_responses(&[(delete_response, "Delete"), (cancel_response, "Cancel")]);
        dialog.set_response_appearance(&delete_response, ResponseAppearance::Destructive);
        dialog.set_response_appearance(&cancel_response, ResponseAppearance::Suggested);

        // get the response from the dialog
        let response = dialog.choose_future(self).await;

        if response == cancel_response {
            return;
        } else if response == delete_response {
            self.imp().listbox.selected_rows().iter().for_each(|row| {
                let dup_file_row = row.downcast_ref::<DupFileRow>().expect("Error getting row");
                let binding = dup_file_row.imp().filepath.label();
                let file_path = binding.as_str();
                std::fs::remove_file(file_path).expect("Error deleting file");
                self.imp().listbox.remove(row);
            });
            // get rid of duplicates with only one file
            self.imp()
                .duplicates_vec
                .borrow_mut()
                .retain(|files| files.len() > 1);

            let toast = Toast::builder()
                .timeout(3000)
                .title("Files deleted successfully")
                .build();
            self.imp().toastoverlay.add_toast(toast);
            self.present_duplicates();
        }
    }

    fn display_filetype_analysis(&self) {
        let mut names_for_background = vec![
            "faint-red",
            "faint-blue",
            "faint-green",
            "faint-yellow",
            "faint-purple",
        ]
        .into_iter();

        let mut analysis = StorageAnalysis::new();
        let home_dir = dirs::download_dir().expect("No home directory");
        analysis.analyse(&home_dir);

        let recents_box = RecentsBox::new();
        self.imp().recents_and_graph.append(&recents_box);

        if analysis.file_types_info.is_empty() {
            return;
        }

        for (file_type, (size, count)) in analysis.file_types_info.iter() {
            let file_type_box = FileTypeBox::new();
            file_type_box.set_property("name", names_for_background.next().expect("No more names"));
            file_type_box
                .imp()
                .file_type_label
                .set_label(file_type.into());
            let count = format_number(*count);
            file_type_box
                .imp()
                .file_count_label
                .set_label(&format!("{} items", count));
            file_type_box.imp().size_label.set_label(&format!(
                "{} of {}",
                format_size(*size),
                format_size(analysis.total_device_memory)
            ));
            file_type_box
                .imp()
                .size_progress
                .set_fraction(*size as f64 / analysis.total_device_memory as f64);
            file_type_box
                .imp()
                .card_icon
                .set_from_file(Some(file_type.get_image_icon()));
            self.imp().file_type_boxes.append(&file_type_box);
        }
    }

    fn save_auth_response_data(&self, auth_response: AuthResponse) {
        self.settings()
            .set_string("token", &auth_response.token)
            .expect("Error setting token");
        self.settings()
            .set_string("user-id", &auth_response.user_id)
            .expect("Error setting user-id");
        self.settings()
            .set_boolean("is-authenticated", true)
            .expect("Error setting is-authenticated");
        self.settings()
            .set_boolean("first-time", false)
            .expect("Error setting first-time");
        self.settings()
            .set_string("username", self.imp().signup_username.text().as_str())
            .expect("Error setting username");
        self.settings()
            .set_string("email", self.imp().signup_email.text().as_str())
            .expect("Error setting email");
    }

    /// Setup the categories
    fn setup_categories(&self) {
        self.imp().main_navigation_view.connect_pushed(clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                let curent_page = window
                    .imp()
                    .main_navigation_view
                    .visible_page()
                    .expect("No page found");
                let current_page_tag = curent_page.tag().expect("No tag found");
                if current_page_tag == "categories" {
                    window.display_categories();
                }
            }
        ));
    }

    /// Display the categories
    fn display_categories(&self) {
        let mut css_names = vec![
            "assignments",
            "projects",
            "notes",
            "exams",
            "study-materials",
            "research-papers",
            "personal-notes",
            "class-schedules",
            "extra-curricular-activities",
        ]
        .into_iter();

        // let categories = Category::VALUES;
        self.imp().categories_flowbox.remove_all();
        for _ in 0..9 {
            let category_box = CategoryBox::new();
            category_box.set_property("name", css_names.next().expect("No more names"));
            self.imp().categories_flowbox.append(&category_box);
        }
    }
}
