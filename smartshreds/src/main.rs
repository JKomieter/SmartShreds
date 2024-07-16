mod window;
mod utils;
mod errors;
mod types;
mod dup_file_row;
mod dup_file_object;
// mod storage_analysis;


use gtk::{gdk::Display, gio, glib, CssProvider};
use adw::prelude::*;
use window::SmartShredsWindow;

const APP_ID: &str = "org.gtk_rs.SmartShreds";

fn main() -> glib::ExitCode {
    // register and include the resources
    gio::resources_register_include!("smartshreds_template.gresource").expect("Failed to include resources");

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/org/gtk_rs/SmartShreds/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider."), 
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}


fn build_ui(app: &adw::Application) {
    let window = SmartShredsWindow::new(app);
    window.present();
}