use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DupFile {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, Default)]
pub struct AuthSettings {
    pub token: String,
    pub username: String,
    pub email: String,
    pub is_authenticated: bool,
    pub client_id: String,
    pub first_time: bool,
}