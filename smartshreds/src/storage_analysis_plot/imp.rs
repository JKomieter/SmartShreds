
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib::{self, Properties};

#[derive(Default, Properties)]
#[properties(wrapper_type = super::StorageAnalysisPlot)]
pub struct StorageAnalysisPlot {
    
}

#[glib::object_subclass]
impl ObjectSubclass for StorageAnalysisPlot {
    const NAME: &'static str = "StorageAnalysisPlot";
    type Type = super::StorageAnalysisPlot;
    type ParentType = gtk::Widget;
}

#[glib::derived_properties]
impl ObjectImpl for StorageAnalysisPlot {}

impl WidgetImpl for StorageAnalysisPlot {}