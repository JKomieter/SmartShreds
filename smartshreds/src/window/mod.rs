mod imp;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use sha2::{Digest, Sha256};
use adw::{prelude::*, AlertDialog, ResponseAppearance};
use gtk::gio::Cancellable;
use gtk::{gio, glib, FileDialog, ListBoxRow, Spinner};
use glib::clone;
use glib::Object;
use adw::subclass::prelude::*;

use crate::dup_file_object::DupFileObject;
use crate::dup_file_row::DupFileRow;
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
            clone!(@weak self as window => move |folder| {
                let folder_name = folder.expect("No folder selected");
                let path = folder_name.path().expect("No path found");
                let number_of_files = utils::number_of_dir_files(&path).expect("Error counting files");
                glib::spawn_future_local(
                    clone!(@weak window as window => async move {
                        window.scan_progress(&path, number_of_files).await;  
                    })
                );
            })
        );
    }

    fn get_duplicates(&self) -> Vec<Vec<DupFile>> {
        self.imp().duplicates_vec.borrow().clone()
    }

    /// Show alert dialog waiting for the scan to complete.
    async fn scan_progress(&self, path: &PathBuf, _number_of_files: u64) {
        let spinner = Spinner::builder().spinning(false).tooltip_text("Scanning...").build();
        
        let cancel_response = "cancel";
        let start_response = "start";
        let dialog = AlertDialog::builder()
            .heading("Scanning in progress")
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
                            file_name: path.file_name().expect("Error getting file name").to_string_lossy().to_string(),
                            file_size: file.metadata().expect("Error getting metadata").len(),
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

            let mut duplicates: Vec<Vec<DupFile>> = Vec::new();
            for (_, files) in duplicates_map {
                if files.len() > 1 {
                    duplicates.push(files);
                }
            }
            
            self.imp().duplicates_vec.replace(duplicates);
            self.imp().page_number.set(1);
            self.present_duplicates();
        }
    }

    fn present_duplicates(&self) {
        self.imp().listbox.remove_all();
        let page_number = if self.imp().page_number.get() == self.imp().duplicates_vec.borrow().len() {
            self.imp().page_number.get()
        } else {
            self.imp().page_number.get() % self.imp().duplicates_vec.borrow().len()
        };
        println!("The current page number is: {}", page_number);
        let duplicate_vec = &self.get_duplicates()[page_number - 1];
        self.imp().pagination.set_label(&format!("{}/{}", page_number, self.imp().duplicates_vec.borrow().len()));
        
        for dup_file in duplicate_vec {
            let list_row_row = self.create_dup_row(dup_file);
            self.imp().listbox.prepend(&list_row_row);
        }
    }

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
}


