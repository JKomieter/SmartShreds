mod imp;

use std::collections::HashMap;
use std::path::PathBuf;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::gio::Settings;
use gtk::glib::clone;
use gtk::{
    gio, glib, CustomFilter, FilterListModel, ListItem, MultiSelection, SignalListItemFactory,
};

use crate::ui::duplicate::duplicate_object::{self, DuplicateObject};
use crate::ui::duplicate::duplicate_row::DuplicateRow;
use crate::ui::file_type_box::FileTypeBox;
use crate::ui::recents_box::RecentsBox;
use crate::utils::duplicates::{traverse_directory_for_duplicates, DupFile, DuplicateFilterMode};
use crate::utils::preview::Preview;
use crate::utils::recents::watch;
use crate::utils::{
    analysis::StorageAnalysis,
    auth::{AuthResponse, AuthSettings},
};
use crate::utils::{format_number, format_size};

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
        self.setup_filetype_analysis();

        self.listen_recents();

        self.setup_duplicates();
        self.get_duplicates();
        self.setup_factory();
        self.show_preview();
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
            self.imp().stack.set_visible_child_name("auth");
        }
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

    fn setup_filetype_analysis(&self) {
        let download_dir = dirs::download_dir().expect("No download directory");
        let document_dir = dirs::document_dir().expect("No document directory");
        let desktop_dir = dirs::desktop_dir().expect("No desktop directory");
        let audio_dir = dirs::audio_dir().expect("No audio directory");
        let video_dir = dirs::video_dir().expect("No video directory");

        let dirs_vec = vec![
            download_dir,
            document_dir,
            desktop_dir,
            audio_dir,
            video_dir,
        ];

        let (sender, receiver) = async_channel::unbounded();

        gio::spawn_blocking(move || {
            let mut combined_analysis = StorageAnalysis::new();
            dirs_vec.iter().for_each(|dir| {
                combined_analysis.analyse(&dir);
            });
            sender
                .send_blocking(combined_analysis)
                .expect("Error sending analysis");
        });

        glib::spawn_future_local(clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                while let Ok(analysis) = receiver.recv().await {
                    window.display_filetype_analysis(analysis);
                }
            }
        ));
    }

    fn display_filetype_analysis(&self, analysis: StorageAnalysis) {
        let recents_box = RecentsBox::new();
        self.imp().recents_and_graph.append(&recents_box);

        if analysis.file_types_info.is_empty() {
            return;
        }

        let mut names_for_background = vec![
            "faint-red",
            "faint-blue",
            "faint-green",
            "faint-yellow",
            "faint-purple",
        ]
        .into_iter();

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

    fn listen_recents(&self) {
        let download_dir = dirs::download_dir().expect("No download directory");
        let document_dir = dirs::document_dir().expect("No document directory");
        let desktop_dir = dirs::desktop_dir().expect("No desktop directory");
        let audio_dir = dirs::audio_dir().expect("No audio directory");
        let video_dir = dirs::video_dir().expect("No video directory");

        let dirs_vec = vec![
            download_dir,
            document_dir,
            desktop_dir,
            audio_dir,
            video_dir,
        ];

        gio::spawn_blocking(move || {
            dirs_vec.iter().for_each(|dir| {
                watch(dir);
            });
        });
    }

    fn duplicates(&self) -> gio::ListStore {
        self.imp()
            .duplicates
            .borrow()
            .clone()
            .expect("Could not get duplicates.")
    }

    fn setup_duplicates(&self) {
        let model = gio::ListStore::new::<DuplicateObject>();
        self.imp().duplicates.replace(Some(model));
        let filter_model = FilterListModel::new(Some(self.duplicates()), self.filters());
        let selection_mode = MultiSelection::new(Some(filter_model));
        self.imp().duplicates_list.set_model(Some(&selection_mode));
    }

    fn get_duplicates(&self) {
        let download_dir = dirs::download_dir().expect("No download directory");
        let document_dir = dirs::document_dir().expect("No document directory");
        let desktop_dir = dirs::desktop_dir().expect("No desktop directory");
        let picture_dir = dirs::picture_dir().expect("No picture directory");
        let audio_dir = dirs::audio_dir().expect("No audio directory");
        let video_dir = dirs::video_dir().expect("No video directory");

        let dirs_vec = vec![
            download_dir,
            // document_dir,
            // desktop_dir,
            // picture_dir,
            // audio_dir,
            // video_dir,
        ];

        let (sender, receiver) = async_channel::unbounded();

        gio::spawn_blocking(move || {
            let mut final_duplicates_map: HashMap<String, Vec<DupFile>> = HashMap::new();
            let mut final_total_file_count = 0;
            let mut final_duplicates_size = 0;
            let mut final_duplicates_count = 0;

            dirs_vec.iter().for_each(|dir| {
                let (duplicates_map, total_file_count, duplicates_size, duplicates_count) =
                    traverse_directory_for_duplicates(dir.clone());
                final_duplicates_map.extend(duplicates_map);
                final_total_file_count += total_file_count;
                final_duplicates_size += duplicates_size;
                final_duplicates_count += duplicates_count;
            });

            sender
                .send_blocking((
                    final_duplicates_map,
                    final_total_file_count,
                    final_duplicates_size,
                    final_duplicates_count,
                ))
                .expect("Error sending duplicates");
        });

        glib::spawn_future_local(clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                while let Ok((
                    duplicates_map,
                    total_file_count,
                    duplicates_size,
                    duplicates_count,
                )) = receiver.recv().await
                {
                    window
                        .imp()
                        .files_scanned
                        .set_label(&format_number(total_file_count));
                    window
                        .imp()
                        .duplicates_count
                        .set_label(&format_number(duplicates_count));

                    window
                        .imp()
                        .duplicates_space_taken
                        .set_label(&format_size(duplicates_size));

                    let mut toggle = false;
                    duplicates_map.into_iter().for_each(|(_, dup_files)| {
                        let bgcolor = if toggle {
                            "blue-duplicate-row"
                        } else {
                            "normal-duplicate-row"
                        };
                        dup_files.into_iter().for_each(|dup_file| {
                            window.add_duplicate_row(dup_file, bgcolor);
                        });
                        toggle = !toggle;
                    });
                }
            }
        ));
    }

    fn add_duplicate_row(&self, dup_file: DupFile, bgcolor: &str) {
        let formated_size = format_size(dup_file.file_size);
        let formatted_date = dup_file
            .date_created
            .format("%-m/%-d/%Y %-I:%M:%S %p")
            .to_string();
        let duplicate_object = DuplicateObject::new(
            dup_file.file_name,
            formated_size,
            dup_file.file_path.to_string_lossy().to_string(),
            formatted_date,
            bgcolor.to_string(),
        );

        self.duplicates().append(&duplicate_object);
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let row = DuplicateRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be a `ListItem`.")
                .set_child(Some(&row));
        });

        factory.connect_bind(move |_, list_item| {
            let duplicate_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be a `ListItem`.")
                .item()
                .and_downcast::<DuplicateObject>()
                .expect("Needs to be a `DuplicateObject`.");

            let duplicate_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be a `ListItem`.")
                .child()
                .and_downcast::<DuplicateRow>()
                .expect("Needs to be a `DuplicateRow`.");

            duplicate_row.bind(&duplicate_object);
        });

        self.imp().duplicates_list.set_factory(Some(&factory));
    }

    fn filters(&self) -> Option<CustomFilter> {
        let filter_modes = self.imp().filter_modes.borrow().clone();

        // if `filter_modes` is just empty, return `None`
        if filter_modes.is_empty() {
            return None;
        }

        let filter = CustomFilter::new(move |obj| {
            let duplicate_object = obj
                .downcast_ref::<DuplicateObject>()
                .expect("Not a DuplicateObject");
            let file_path: PathBuf = duplicate_object.path().into();
            let file_extension = file_path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");
            let duplicate_file_type = DuplicateFilterMode::from_extension(file_extension);

            // only allow duplicates that are of the same type as those in the filter_modes
            filter_modes.contains(&duplicate_file_type)
        });

        Some(filter)
    }

    fn apply_filters(&self) {
        let filter_model = FilterListModel::new(Some(self.duplicates()), self.filters());
        let selection_mode = MultiSelection::new(Some(filter_model));
        self.imp().duplicates_list.set_model(Some(&selection_mode));

        let duplicates_count = selection_mode.n_items();
        self.imp()
            .duplicates_count
            .set_label(&format_number(duplicates_count.into()));

        let duplicates = self.duplicates();
        let mut size = 0;
        let mut position = 0;

        while let Some(item) = duplicates.item(position) {
            let duplicate_object = item
                .downcast_ref::<DuplicateObject>()
                .expect("Not a DuplicateObject");
            let duplicate_size = duplicate_object
                .size()
                .split(" ")
                .next()
                .unwrap_or("0")
                .parse::<u64>()
                .unwrap_or(0);
            size += duplicate_size;
            position += 1;
        }
        // Todo: Fix this
        self.imp()
            .duplicates_space_taken
            .set_label(&format_size(size));
    }

    fn show_preview(&self) {
        self.imp().duplicates_list.connect_activate(clone!(
            #[weak(rename_to = window)]
            self,
            move |_, i| {
                let duplicates = window.duplicates();
                if let Some(item) = duplicates.item(i) {
                    let duplicate_object = item
                        .downcast_ref::<DuplicateObject>()
                        .expect("Not a DuplicateObject");
                    let path = duplicate_object.path();
                    let preview = Preview::new(path.into());
                    if let Some(widget) = preview.widget() {
                        window.imp().preview_viewport.set_child(Some(&widget));
                    }
                }
            }
        ));
    }
}
