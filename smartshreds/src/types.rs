use std::path::PathBuf;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone)]
pub struct DupFile {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
}

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

#[derive(Debug)]
pub enum Category {
    Assignments,
    Projects,
    Exams,
    Notes,
    StudyMaterials,
    ResearchPapers,
    PersonalNotes,
    ClassSchedules,
    ExtraCurricularActivities,
}

impl Category {
    pub const VALUES: [Self; 9] = [
        Self::Assignments,
        Self::Projects,
        Self::Exams,
        Self::Notes,
        Self::StudyMaterials,
        Self::ResearchPapers,
        Self::PersonalNotes,
        Self::ClassSchedules,
        Self::ExtraCurricularActivities,
    ];
}