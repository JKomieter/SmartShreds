mod imp;

use adw::subclass::prelude::*;
use adw::{prelude::*, AlertDialog, ResponseAppearance, Toast};
use glib::clone;
use glib::Object;
use gtk::gio::Cancellable;
use gtk::{gio, glib, FileDialog, Label, ListBoxRow, Spinner};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::types::DupFile;
use crate::ui::dup_file_object::DupFileObject;
use crate::ui::dup_file_row::DupFileRow;
use crate::ui::file_type_box::FileTypeBox;
use crate::ui::recents_box::RecentsBox;
use crate::utils::analysis::StorageAnalysis;
use crate::utils::{format_number, format_size, row_tooltip_markup};

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
            let mut dir_queue: Vec<PathBuf> = vec![path.to_path_buf()];
            let mut duplicates_map: HashMap<String, Vec<DupFile>> = HashMap::new();
            let mut hasher = Sha256::new();
            // scan the directory
            while let Some(dir) = dir_queue.pop() {
                for entry in std::fs::read_dir(&dir).expect("Error reading directory") {
                    let entry = entry.expect("Error reading entry");
                    let path = entry.path();
                    if path.is_dir() {
                        dir_queue.push(path);
                    } else {
                        let mut file = File::open(&path).expect("Error opening file");
                        let dup_file = DupFile {
                            file_path: path.clone(),
                            file_name: path
                                .file_name()
                                .expect("Error getting file name")
                                .to_string_lossy()
                                .to_string(),
                            file_size: file.metadata().expect("Error getting metadata").len(),
                        };
                        let mut contents = Vec::new();
                        file.read_to_end(&mut contents).expect("Error reading file");
                        hasher.update(&contents);
                        let result = hasher.finalize_reset();
                        let hash = format!("{:x}", result);
                        duplicates_map
                            .entry(hash)
                            .or_insert_with(Vec::new)
                            .push(dup_file);
                    }
                }
            }
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

    fn display_analysis(&self) {
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
}
