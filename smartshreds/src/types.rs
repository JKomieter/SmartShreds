use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DupFile {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
}

