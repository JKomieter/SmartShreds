use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{
    self, gio::Settings, glib::{self, clone, subclass::InitializingObject}, Box, Button, CompositeTemplate, Label, ListBoxRow, Stack
};
use std::{cell::{RefCell, OnceCell}, collections::HashMap};

use crate::utils::{auth::AuthResponse, duplicates::DuplicateFilterMode, runtime};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/SmartShreds/window.ui")]
pub struct SmartShredsWindow {
    #[template_child]
    pub main_navigation_view: TemplateChild<adw::NavigationView>,

    // settings
    pub settings: OnceCell<Settings>,
    #[template_child]
    pub stack: TemplateChild<Stack>,

    // home page
    #[template_child]
    pub file_type_boxes: TemplateChild<Box>,
    #[template_child]
    pub recents_and_graph: TemplateChild<Box>,

    // duplicates page
    #[template_child]
    pub files_scanned: TemplateChild<Label>,
    #[template_child]
    pub duplicates_count: TemplateChild<Label>,
    #[template_child]
    pub duplicates_space_taken: TemplateChild<Label>,
    #[template_child]
    pub duplicates_filter_box: TemplateChild<Box>,
    pub filter_modes: RefCell<Vec<DuplicateFilterMode>>,

    // onboarding page
    #[template_child]
    pub auth_navigation_view: TemplateChild<adw::NavigationView>,
    #[template_child]
    pub role: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub feature: TemplateChild<gtk::DropDown>,
    #[template_child]
    pub recv_notifications: TemplateChild<gtk::Switch>,
    #[template_child]
    pub signup_username: TemplateChild<gtk::Entry>,
    #[template_child]
    pub signup_email: TemplateChild<gtk::Entry>,
    #[template_child]
    pub signup_password: TemplateChild<gtk::PasswordEntry>,
    #[template_child]
    pub signup_confirm_password: TemplateChild<gtk::PasswordEntry>,
    // recents
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
        obj.setup_settings();
        // check if user is logged in.
        // obj.check_authentication();
        // if !obj.settings().boolean("is-authenticated") {
        //     return;
        // }
        obj.setup();
    }
}

#[gtk::template_callbacks]
impl SmartShredsWindow {
    /// Filter the duplicates.
    #[template_callback]
    fn filter_duplicates(&self, button: &Button) {
        let css_classes: Vec<String> = button
            .css_classes()
            .iter()
            .map(|c| c.as_str().to_string())
            .collect();
        let label = button.label().expect("Button has no label").to_string();
        let new_mode = DuplicateFilterMode::from(label.as_str());
        let filter_modes = self.filter_modes.borrow();
        if css_classes.contains(&"suggested-action".to_string()) {
            // remove the filter mode.
            let index = filter_modes.iter().position(|m| m == &new_mode).unwrap();
            self.filter_modes.borrow_mut().remove(index);
            button.remove_css_class("suggested-action");
        } else {
            // add the filter mode.
            self.filter_modes.borrow_mut().push(new_mode);
            button.add_css_class("suggested-action");
        }
    }

    /// The navigation row is clicked.
    #[template_callback]
    fn on_navrow_activated(&self, listboxrow: &ListBoxRow) {
        let index = listboxrow.index();
        // match the index of the row selected to the navigation page tag.
        let tag = match index {
            1 => "home",
            2 => "categories",
            3 => "duplicates",
            _ => "home",
        };
        self.main_navigation_view.pop();
        self.main_navigation_view.push_by_tag(tag);
    }

    #[template_callback]
    fn handle_signup_clicked(&self) {
        self.auth_navigation_view.push_by_tag("loading");

        let role = match self.role.selected() {
            0 => "Student",
            1 => "Educator",
            3 => "Researcher",
            _ => "Other",
        };
        let feature = match self.feature.selected() {
            0 => "File organization",
            1 => "Duplicate detection",
            2 => "Reducing clutter",
            _ => "Other",
        };
        let recv_notifications = self.recv_notifications.is_active();
        let username = self.signup_username.text().to_string();
        let email = self.signup_email.text().to_string();
        let password = self.signup_password.text().to_string();
        let confirm_password = self.signup_confirm_password.text().to_string();

        if username.is_empty()
            || email.is_empty()
            || password.is_empty()
            || confirm_password.is_empty()
            || password != confirm_password
        {
            return;
        }

        const OS: &str = std::env::consts::OS;

        let (sender, receiver) = async_channel::bounded(1);
        // signup the user.
        runtime().spawn(clone!(
            #[strong]
            sender,
            async move {
                let mut body = HashMap::new();
                body.insert("username", username);
                body.insert("email", email);
                body.insert("password", password);
                body.insert("role", role.to_string());
                body.insert("feature", feature.to_string());
                body.insert("recv_notifications", recv_notifications.to_string());
                body.insert("OS", OS.to_string());

                let response = reqwest::Client::new()
                    .post("https://smartshreds-e99e6de33f8d.herokuapp.com/signup")
                    .json(&body)
                    .send()
                    .await;
                sender.send(response).await.expect("Error sending response");
            }
        ));

        // wait for the response.
        glib::spawn_future_local(clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                while let Ok(response) = receiver.recv().await {
                    if let Ok(response) = response {
                        if response.status().is_success() {
                            let auth_response = response
                                .json::<AuthResponse>()
                                .await
                                .expect("Error parsing response");
                            window.obj().save_auth_response_data(auth_response);
                            window.stack.set_visible_child_name("main");
                        }
                    }
                }
            }
        ));
    }

    #[template_callback]
    fn handle_signin_clicked(&self) {}
}

impl WidgetImpl for SmartShredsWindow {}

impl ApplicationWindowImpl for SmartShredsWindow {}

impl AdwApplicationWindowImpl for SmartShredsWindow {}

impl WindowImpl for SmartShredsWindow {}
