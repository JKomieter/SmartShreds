pub mod analysis;
pub mod recents;
pub mod auth;
pub mod duplicates;
pub mod preview;

use std::sync::OnceLock;
use tokio::runtime::Runtime;




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
