use chrono::prelude::*;
use std::fs;
use std::rc::Rc;


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
