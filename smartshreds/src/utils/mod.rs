use std::path::PathBuf;
use crate::errors::SmartShredsError;

/// get the number of files in a directory
pub fn number_of_dir_files(path: &PathBuf) -> Result<u64, SmartShredsError> {
    let mut dir_queue: Vec<PathBuf> = vec![path.to_path_buf()];
    let mut file_count = 0;

    while let Some(dir) = dir_queue.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dir_queue.push(path);
            } else {
                file_count += 1;
            }
        }
    }

    Ok(file_count)
}

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
