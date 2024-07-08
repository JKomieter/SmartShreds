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

