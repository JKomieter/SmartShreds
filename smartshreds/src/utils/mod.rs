use crate::errors::SmartShredsError;
use chrono::prelude::*;
use std::fs;
use std::{path::PathBuf, rc::Rc};

/// get the number of files in a directory
pub fn number_of_dir_files(path: &PathBuf) -> Result<u64, SmartShredsError> {
    let mut dir_queue: Vec<PathBuf> = vec![path.to_path_buf()];
    let mut file_count = 0;

    while let Some(entry) = dir_queue.pop() {
        if entry.is_dir() {
            dir_queue.push(entry.to_path_buf());
        } else {
            file_count += 1;
        }
    }

    Ok(file_count)
}

/// Get the size of a file
// #[inline]
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

/// Get the tooltip markup for a row
// #[inline]
pub fn row_tooltip_markup(file_path: &str) -> String {
    let metadata = fs::metadata(file_path).expect("Error getting file metadata");

    let modified_time = metadata
        .modified()
        .expect("Error getting file modified time");
    let datetime: DateTime<Utc> = modified_time.into();
    let formatted_modified_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    let file_size = format_size(metadata.len());

    let last_accessed_time = metadata
        .accessed()
        .expect("Error getting file last accessed time");
    let datetime: DateTime<Utc> = last_accessed_time.into();
    let formatted_last_accessed_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    // Getting creating time is not supported on all platforms
    let mut created_meta = Rc::new("".to_string());

    if let Ok(_time) = metadata.created() {
        let created_time: DateTime<Local> = metadata.created().unwrap().into();
        created_meta = Rc::new(created_time.format("%Y-%m-%d %H:%M:%S").to_string());
    }
    format!(
        "Time created: {}\nTime modified: {}\nTime last accessed: {}\nSize: {}",
        Rc::clone(&created_meta),
        formatted_modified_date,
        formatted_last_accessed_date,
        file_size
    )
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

    #[test]
    fn test_row_tooltip_markup() {
        let file_path = "test_dir/a.txt";
        let metadata = fs::metadata(file_path).expect("Error getting file metadata");

        assert!(row_tooltip_markup(file_path).contains(&metadata.len().to_string()));
    }
    
}
