use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct AuthSettings {
    pub token: String,
    pub username: String,
    pub email: String,
    pub is_authenticated: bool,
    pub first_time: bool,
    pub user_id: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
}
