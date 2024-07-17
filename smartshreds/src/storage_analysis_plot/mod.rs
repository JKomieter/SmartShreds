pub mod analysis;
mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct StorageAnalysisPlot(ObjectSubclass<imp::StorageAnalysisPlot>)
        @extends gtk::Widget;
}


impl StorageAnalysisPlot {
    pub fn new() -> Self {
        Object::builder().build()
    }
}