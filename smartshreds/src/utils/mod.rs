pub mod analysis;
pub mod recents;
pub mod auth;
pub mod duplicates;

use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::OnceLock;
use tokio::runtime::Runtime;


#[derive(Debug, Clone)]
pub struct DupFile {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size: u64,
}

/// Get the size of a file
#[inline]
pub fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1048576 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else if size < 1073741824 {
        format!("{:.2} MB", size as f64 / 1048576.0)
    } else {
        format!("{:.2} GB", size as f64 / 1073741824.0)
    }
}

pub fn format_number(number: u64) -> String {
    if number < 1000 {
        return number.to_string();
    }
    let mut n = number;
    let mut count = 0;
    while n >= 1000 {
        n /= 1000;
        count += 1;
    }
    let suffix = match count {
        1 => "K",
        2 => "M",
        3 => "B",
        4 => "T",
        _ => "E",
    };
    format!("{}{}", n, suffix)
}



pub fn traverse_directory_for_duplicates(path: PathBuf) -> HashMap<String, Vec<DupFile>> {
    let mut dir_queue: Vec<PathBuf> = vec![path];
    let mut duplicates_map: HashMap<String, Vec<DupFile>> = HashMap::new();
    let mut hasher = Sha256::new();
    // scan the directory
    while let Some(dir) = dir_queue.pop() {
        for entry in std::fs::read_dir(&dir).expect("Error reading directory") {
            let entry = entry.expect("Error reading entry");
            let path = entry.path();
            if path.is_dir() {
                dir_queue.push(path);
            } else {
                let mut file = File::open(&path).expect("Error opening file");
                let dup_file = DupFile {
                    file_path: path.clone(),
                    file_name: path
                        .file_name()
                        .expect("Error getting file name")
                        .to_string_lossy()
                        .to_string(),
                    file_size: file.metadata().expect("Error getting metadata").len(),
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

    duplicates_map
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1048576), "1.00 MB");
        assert_eq!(format_size(1073741824), "1.00 GB");
    }
}

pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}
