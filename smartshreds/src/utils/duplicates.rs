use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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

#[derive(Debug, Clone)]
pub struct DupFile {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
    pub date_created: DateTime<Utc>,
}

pub fn traverse_directory_for_duplicates(path: PathBuf) -> (
    HashMap<String, Vec<DupFile>>,
    u64,
    u64,
    u64,
) {
    let mut dir_queue: Vec<PathBuf> = vec![path];
    let mut duplicates_map: HashMap<String, Vec<DupFile>> = HashMap::new();
    let mut hasher = Sha256::new();
    let mut total_file_count = 0;
    let mut duplicates_size = 0;
    let mut duplicates_count = 0;
    // scan the directory
    while let Some(dir) = dir_queue.pop() {
        for entry in std::fs::read_dir(&dir).expect("Error reading directory") {
            let entry = entry.expect("Error reading entry");
            let path = entry.path();
            if path.is_dir() {
                dir_queue.push(path);
            } else {
                total_file_count += 1;
                let mut file = File::open(&path).expect("Error opening file");
                let dup_file = DupFile {
                    file_path: path.clone(),
                    file_name: path
                        .file_name()
                        .expect("Error getting file name")
                        .to_string_lossy()
                        .to_string(),
                    file_size: file.metadata().expect("Error getting metadata").len(),
                    date_created: file
                        .metadata()
                        .expect("Error getting metadata")
                        .created()
                        .expect("Error getting created")
                        .into(),
                };
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).expect("Error reading file");
                hasher.update(&contents);
                let result = hasher.finalize_reset();
                let hash = format!("{:x}", result);
                duplicates_map
                    .entry(hash)
                    .or_insert_with(Vec::new)
                    .push(dup_file);
            }
        }
    }

    let final_duplicates_map: HashMap<String, Vec<DupFile>> = duplicates_map.into_iter().filter(|(_, v)| v.len() > 1).collect();
    final_duplicates_map.iter().for_each(|(_, v)| {
        v.iter().for_each(|f| {
            duplicates_size += f.file_size;
            duplicates_count += 1;
        });
    });

    println!("Total files: {}", total_file_count);
    println!("Duplicates size: {}", duplicates_size);
    println!("Duplicates count: {}", duplicates_count);

    (final_duplicates_map, total_file_count, duplicates_size, duplicates_count)
}
