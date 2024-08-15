use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum DuplicateFilterMode {
    Images,
    Documents,
    Videos,
    Audios,
}

impl From<&str> for DuplicateFilterMode {
    fn from(mode: &str) -> Self {
        match mode {
            "Images" => DuplicateFilterMode::Images,
            "Documents" => DuplicateFilterMode::Documents,
            "Videos" => DuplicateFilterMode::Videos,
            "Audios" => DuplicateFilterMode::Audios,
            _ => DuplicateFilterMode::Documents,
        }
    }
}

impl DuplicateFilterMode {
    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "txt" | "doc" | "docx" | "pdf" | "ppt" | "pptx" | "xls" | "xlsx" => {
                DuplicateFilterMode::Documents
            }
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => DuplicateFilterMode::Images,
            "mp4" | "avi" | "mkv" | "flv" | "wmv" | "mov" => DuplicateFilterMode::Videos,
            "mp3" | "wav" | "flac" | "aac" | "ogg" => DuplicateFilterMode::Audios,
            "js" | "html" | "css" | "py" | "go" | "java" | "cpp" | "c" | "h" | "hpp" | "rs"
            | "ts" | "json" | "xml" | "yaml" | "toml" => DuplicateFilterMode::Documents,
            _ => DuplicateFilterMode::Documents,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DupFile {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
    pub date_created: DateTime<Utc>,
}

pub fn traverse_directory_for_duplicates(
    path: PathBuf,
) -> (HashMap<String, Vec<DupFile>>, u64, u64, u64) {
    let mut dir_queue: Vec<PathBuf> = vec![path];
    let mut duplicates_map: HashMap<String, Vec<DupFile>> = HashMap::new();
    let mut hasher = Sha256::new();
    let mut total_file_count = 0;
    let mut duplicates_size = 0;
    let mut duplicates_count = 0;

    while let Some(dir) = dir_queue.pop() {
        match fs::read_dir(&dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_dir() {
                                dir_queue.push(path);
                            } else {
                                total_file_count += 1;
                                match File::open(&path) {
                                    Ok(mut file) => {
                                        let dup_file = DupFile {
                                            file_path: path.clone(),
                                            file_name: path
                                                .file_name()
                                                .expect("Error getting file name")
                                                .to_string_lossy()
                                                .to_string(),
                                            file_size: file
                                                .metadata()
                                                .expect("Error getting metadata")
                                                .len(),
                                            date_created: file
                                                .metadata()
                                                .expect("Error getting metadata")
                                                .created()
                                                .expect("Error getting created")
                                                .into(),
                                        };
                                        let mut contents = Vec::new();
                                        match file.read_to_end(&mut contents) {
                                            Ok(_) => {
                                                hasher.update(&contents);
                                                let result = hasher.finalize_reset();
                                                let hash = format!("{:x}", result);
                                                duplicates_map
                                                    .entry(hash)
                                                    .or_insert_with(Vec::new)
                                                    .push(dup_file);
                                            }
                                            Err(e) => {
                                                eprintln!("Error reading file: {}", e);
                                                continue;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error opening file: {}", e);
                                        continue;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading entry in {:?}: {}", dir, e);
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading directory: {}", e);
                continue;
            }
        }
    }

    let final_duplicates_map: HashMap<String, Vec<DupFile>> = duplicates_map
        .into_iter()
        .filter(|(_, v)| v.len() > 1)
        .collect();
    final_duplicates_map.iter().for_each(|(_, v)| {
        v.iter().for_each(|f| {
            duplicates_size += f.file_size;
            duplicates_count += 1;
        });
    });

    (
        final_duplicates_map,
        total_file_count,
        duplicates_size,
        duplicates_count,
    )
}
