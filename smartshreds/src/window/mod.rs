mod imp;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use sha2::{Digest, Sha256};
use adw::{prelude::*, AlertDialog};
use gtk::gio::Cancellable;
use gtk::{gio, glib, FileDialog, ProgressBar};
use glib::clone;
use glib::Object;

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

    // Show alert dialog waiting for the scan to complete.
    async fn scan_progress(&self, path: &PathBuf, number_of_files: u64) {
        let progressbar = ProgressBar::builder()
            .text("Scanning...")
            .pulse_step(number_of_files as f64 / 100.0)
            .show_text(true)
            .build();
        let mut dir_queue: Vec<PathBuf> = vec![path.to_path_buf()];
        let mut duplicates_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
        let mut hasher = Sha256::new();
        let mut file_count = 0;

        while let Some(dir) = dir_queue.pop() {
            for entry in std::fs::read_dir(&dir).expect("Error reading directory") {
                let entry = entry.expect("Error reading entry");
                let path = entry.path();
                if path.is_dir() {
                    dir_queue.push(path);
                } else {
                    let mut file = File::open(&path).expect("Error opening file");
                    let mut contents = Vec::new();
                    file.read_to_end(&mut contents).expect("Error reading file");
                    hasher.update(&contents);
                    let result = hasher.finalize_reset();
                    let hash = format!("{:x}", result);
                    duplicates_map.entry(hash).or_insert_with(Vec::new).push(path);
                    file_count += 1;
                }
            }
            progressbar.set_fraction(file_count as f64 / number_of_files as f64);
        }
        
        let dialog = AlertDialog::builder()
            .heading("Scanning in progress")
            .extra_child(&progressbar)
            .build();
        let cancel_response = "cancel";
        dialog.add_response(&cancel_response, "Cancel");
        
        let response = dialog.choose_future(self).await;
        if response == cancel_response {
            return;
        }
    }
}


