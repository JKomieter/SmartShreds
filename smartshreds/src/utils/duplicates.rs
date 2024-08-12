
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum DuplicateFilterMode {
    All, 
    Images,
    Documents,
    Videos,
    Audio,
}

impl From<&str> for DuplicateFilterMode {
    fn from(mode: &str) -> Self {
        match mode {
            "All" => DuplicateFilterMode::All,
            "Images" => DuplicateFilterMode::Images,
            "Documents" => DuplicateFilterMode::Documents,
            "Videos" => DuplicateFilterMode::Videos,
            "Audio" => DuplicateFilterMode::Audio,
            _ => DuplicateFilterMode::All,
        }
    }
}