mod imp;

use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::Object;
use gtk::gio::Settings;
use gtk::{gio, glib};

use crate::ui::file_type_box::FileTypeBox;
use crate::ui::recents_box::RecentsBox;
use crate::utils::recents::watch;
use crate::utils::{
    analysis::StorageAnalysis,
    auth::{AuthResponse, AuthSettings},
};
use crate::utils::{
    format_number, format_size,
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
        self.listen_recents();
        self.setup_duplicates();
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

    fn listen_recents(&self) {
        let download_dir = dirs::download_dir().expect("No download directory");
        let document_dir = dirs::document_dir().expect("No document directory");
        let desktop_dir = dirs::desktop_dir().expect("No desktop directory");
        let audio_dir = dirs::audio_dir().expect("No audio directory");
        let video_dir = dirs::video_dir().expect("No video directory");
        
        let dirs_vec = vec![download_dir, document_dir, desktop_dir, audio_dir, video_dir];

        // dirs_vec.iter().for_each(|dir| {
        //     let dir = dir.clone();
        //     
        // });

        // gio::spawn_blocking(move || {
        //     watch(&dirs_vec[0]);
        // });
    }

    fn setup_duplicates(&self) {
        
    }
}
