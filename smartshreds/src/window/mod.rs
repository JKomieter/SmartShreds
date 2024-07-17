mod imp;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};
use adw::{prelude::*, AlertDialog, ResponseAppearance, Toast, };
use gtk::gio::Cancellable;
use gtk::{gio, glib, FileDialog, Label, ListBoxRow, Spinner};
use glib::clone;
use glib::Object;
use adw::subclass::prelude::*;

use crate::dup_file_object::DupFileObject;
use crate::dup_file_row::DupFileRow;
use crate::file_type_box::FileTypeBox;
use crate::storage_analysis_plot::analysis::StorageAnalysis;
use crate::types::DupFile;
use crate::utils;

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
        let file_dialog = FileDialog::builder().title("Choose directory to scan").build();
        file_dialog.select_folder(
            Some(self), 
            Some(&Cancellable::new()), 
            clone!(#[weak(rename_to = window)] self, move |folder| {
                if let Ok(folder_name) = folder {
                    let path = folder_name.path().expect("No path found");
                    glib::spawn_future_local(
                        clone!(#[weak] window, async move {
                            window.scan_progress(&path).await;  
                        })
                    );
                }
            })
        );
    }

    fn get_duplicates(&self) -> Vec<Vec<DupFile>> {
        self.imp().duplicates_vec.borrow().clone()
    }

    /// Show alert dialog waiting for the scan to complete.
    async fn scan_progress(&self, path: &PathBuf) {
        let spinner = Spinner::builder().spinning(false).tooltip_text("Scanning...").build();
        
        let cancel_response = "cancel";
        let start_response = "start";
        let dialog = AlertDialog::builder()
            .heading("Press start to scan")
            .extra_child(&spinner)
            .close_response(cancel_response)
            .default_response(start_response)
            .build();
        dialog
            .add_responses(&[(cancel_response, "Cancel"), (start_response, "Start")]);
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
                            file_name: path.file_name()
                                .expect("Error getting file name")
                                .to_string_lossy()
                                .to_string(),
                            file_size: file.metadata()
                                .expect("Error getting metadata")
                                .len(),
                        };
                        let mut contents = Vec::new();
                        file.read_to_end(&mut contents).expect("Error reading file");
                        hasher.update(&contents);
                        let result = hasher.finalize_reset();
                        let hash = format!("{:x}", result);
                        duplicates_map.entry(hash).or_insert_with(Vec::new).push(dup_file);
                   
                    }
                }
            }
            // put a check to see if there are any duplicates
            
            self.imp().duplicates_vec.replace(
                duplicates_map
                .values()
                .filter(|files| files.len() > 1)
                .map(|files| files.clone())
                .collect()
            );
            self.imp().page_number.set(1);
            self.present_duplicates();
        }
    }

    /// Present the duplicate files in the listbox.
    fn present_duplicates(&self) {
        self.imp().listbox.remove_all();
        let page_number = if self.imp().page_number.get() == self.imp().duplicates_vec.borrow().len() {
            self.imp().page_number.get()
        } else {
            self.imp().page_number.get() % self.imp().duplicates_vec.borrow().len()
        };
        println!("The current page number is: {}", page_number);
        let duplicate_vec = &self.get_duplicates()[page_number - 1];
        self.imp().pagination.set_label(&format!("{} of {}", page_number, self.imp().duplicates_vec.borrow().len()));
      
        duplicate_vec.iter().for_each(|dup_file| {
            let row = self.create_dup_row(dup_file);
            let tooltip_markup = utils::row_tooltip_markup(&dup_file.file_path.to_string_lossy());
            row.set_tooltip_text(Some(&tooltip_markup));
            self.imp().listbox.append(&row);
        });
        if duplicate_vec.len() > 0 {
            let filesize = &format!("FILE SIZE: {}", utils::format_size(duplicate_vec[0].file_size));
            self.imp().filesize.set_label(filesize);
        }

    }

    /// Create a listbox row for the duplicate
    // #[inline]
    fn create_dup_row(&self, dup_file: &DupFile) -> ListBoxRow {
        let filesize = utils::format_size(dup_file.file_size);
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
        dialog
            .add_responses(&[(delete_response, "Delete"), (cancel_response, "Cancel")]);
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
            self.imp().duplicates_vec.borrow_mut().retain(|files| files.len() > 1);
            
            let toast = Toast::builder()
                .timeout(3000)
                .title("Files deleted successfully")
                .build();
            self.imp().toastoverlay.add_toast(toast);
            self.present_duplicates();
        }
    }

    fn display_file_types(&self) {
        let mut image_file_paths = vec![
            "assets/icons/document.png",
            "assets/icons/image.png",
            "assets/icons/video.png",
            "assets/icons/audio.png",
            "assets/icons/other.png",
        ];

        
    }
}


