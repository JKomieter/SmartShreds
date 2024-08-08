use chrono::{DateTime, Utc};
// use gtk::glib::{self, clone};
// use reqwest::Client;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::PathBuf;
use sysinfo::System;
// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use super::runtime;

#[derive(Debug, Clone, Default)]
pub struct StorageAnalysis {
    // size, count
    pub file_types_info: HashMap<FileType, (u64, u64)>,
    pub inactive_files: Vec<PathBuf>,
    pub junk_files: Vec<JunkFiles>,
    pub memory_usage: MemoryUsage,
    pub total_device_memory: u64,
    pub recent_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryUsage {
    pub size: u64,
    pub total_folders: u64,
    pub total_files: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileType {
    Image,
    Video,
    Audio,
    Document,
    Other,
}

impl From<&FileType> for &str {
    fn from(value: &FileType) -> Self {
        match value {
            FileType::Image => "Image",
            FileType::Video => "Video",
            FileType::Audio => "Audio",
            FileType::Document => "Document",
            FileType::Other => "Other",
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Other
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum JunkFiles {
    RandomExe,
    DeletedFiles,
    TempFiles,
    TempInternetFiles,
    Logs,
    Thumbnails,
    SoftwareLeftovers,
}

impl Default for JunkFiles {
    fn default() -> Self {
        JunkFiles::RandomExe
    }
}

impl From<&str> for FileType {
    fn from(value: &str) -> Self {
        match value {
            "txt" | "doc" | "docx" | "pdf" | "ppt" | "pptx" | "xls" | "xlsx" => FileType::Document,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => FileType::Image,
            "mp4" | "avi" | "mkv" | "flv" | "wmv" | "mov" => FileType::Video,
            "mp3" | "wav" | "flac" | "aac" | "ogg" => FileType::Audio,
            "js" | "html" | "css" | "py" | "go" | "java" | "cpp" | "c" | "h" | "hpp" | "rs"
            | "ts" | "json" | "xml" | "yaml" | "toml" => FileType::Document,
            _ => FileType::Other,
        }
    }
}

impl FileType {
    pub fn get_image_icon(&self) -> &str {
        match self {
            FileType::Image => "assets/icons/image.png",
            FileType::Video => "assets/icons/video.png",
            FileType::Audio => "assets/icons/audio.png",
            FileType::Document => "assets/icons/document.png",
            FileType::Other => "assets/icons/other.png",
        }
    }
}

#[allow(dead_code)]
impl StorageAnalysis {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_device_memory = sys.total_memory();
        let memory_used = sys.used_memory();

        let analysis = StorageAnalysis {
            total_device_memory,
            memory_usage: MemoryUsage {
                size: memory_used,
                ..Default::default()
            },
            ..Default::default()
        };

        analysis
    }

    pub fn analyse(&mut self, start_path: &PathBuf) {
        let mut dir_queue: VecDeque<PathBuf> = VecDeque::new();
        dir_queue.push_back(start_path.to_path_buf());
        let mut unpermitted_dirs: HashSet<PathBuf> = HashSet::new();

        // let (sender, receiver) = async_channel::bounded(1);
        // runtime().spawn(clone!(
        //     #[strong]
        //     sender,
        //     async move {

        //     }
        // ));

        // glib::spawn_future_local(async move {
            
        // });

        while let Some(dir) = dir_queue.pop_front() {
            let dir_parent = dir.parent().expect("Error getting parent directory");
            // skip directories that are not permitted
            if unpermitted_dirs.contains(dir_parent) {
                continue;
            }

            if dir.is_dir() {
                self.memory_usage.total_folders += 1;
                match fs::read_dir(&dir) {
                    Ok(entries) => {
                        for entry in entries.flatten() {
                            dir_queue.push_back(entry.path());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading directory {:?}: {}", dir, e);
                        unpermitted_dirs.insert(dir_parent.to_path_buf());
                        continue;
                    }
                }
            } else {
                self.memory_usage.total_files += 1;
                self.process_file(&dir);
            }
        }
    }

    fn process_file(&mut self, path: &PathBuf) {
        if let Ok(metadata) = fs::metadata(path) {
            let file_size = metadata.len();
            let file_extension = path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");
            let file_type: FileType = file_extension.into();

            let entry = self
                .file_types_info
                .entry(file_type.clone())
                .or_insert((0, 0));
            entry.0 += file_size;
            entry.1 += 1;

            // check for recent created or modified file in the last 7 days and inactive files
            let (created, accessed, modified) = self.get_file_timestamps(path);
            if modified > (Utc::now() - chrono::Duration::days(7))
                || created > (Utc::now() - chrono::Duration::days(7))
                || accessed > (Utc::now() - chrono::Duration::days(7))
            {
                self.recent_files.push(path.clone());
            } else if accessed < (Utc::now() - chrono::Duration::days(365)) {
                self.inactive_files.push(path.clone());
            }

            self.detect_junk_files();
        }
    }

    fn get_file_timestamps(&self, path: &PathBuf) -> (DateTime<Utc>, DateTime<Utc>, DateTime<Utc>) {
        let metadata = fs::metadata(path).expect("Error getting file metadata");
        let modified = metadata
            .modified()
            .map(|t| DateTime::<Utc>::from(t))
            .unwrap();
        let created = metadata
            .created()
            .map(|t| DateTime::<Utc>::from(t))
            .unwrap();
        let accessed = metadata
            .accessed()
            .map(|t| DateTime::<Utc>::from(t))
            .unwrap();
        (created, accessed, modified)
    }

    pub fn is_irrelevant_file(&self, path: &PathBuf) -> bool {
        let irrelevant_file_types = vec![
            // Windows
            "exe", "dll", "tmp", "log", "bak", "old", "chk", "swp", "temp", "thumbs.db", "desktop.ini",
            "lnk", "url", "ini", "db", "dbf", "mdb", "accdb", "sql", "mdf", "ldf", "sdf", "sqlite",
            "sqlite3",
            // MacOS
            "dmg", "pkg", "app", "ipa", "iso", "toast", "dmgpart", "sparseimage", "appex", "xip",
            "pkg", "mpkg", "prefPane", "qlgenerator", "saver", "mdimporter", "workflow", "cpgz",
            "usr", "xar", "xip", "z", "zip", "gz", "tar", "tgz", "tbz", "bz2", "xz", "lz", "lzma",
            // Linux
            "deb", "rpm", "AppImage", "snap", "run", "sh", "bin", "out",
            "o", "a", "so", "ko", "la", "lai", "lo", "po", "mo", "pot", "class", "jar", "war", "ear",
        ];

        return irrelevant_file_types.into_iter().any(|irr| {
            path.to_str().unwrap().contains(irr)
        });
    }

    fn detect_junk_files(&mut self) {}

    pub fn recent_files(&self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_storage_analysis() {
        let mut analysis = StorageAnalysis::new();
        let download_dir = dirs::download_dir().expect("Error getting download directory");
        analysis.analyse(&download_dir);
        assert!(analysis.memory_usage.total_folders > 0);
        assert!(analysis.memory_usage.total_files > 0);

        println!("{:#?}", analysis);
    }

    #[test]
    pub fn text_irrelevant_files() {
        let file_paths = [
            "/home/user/Downloads/file.exe",
            "/home/user/Downloads/file.dmg",
            "/home/user/Downloads/file.deb",
            "/home/user/Downloads/file.txt",
            "'usr/local/bin/file.sh",
            "'home/user/Downloads/file.gz",
        ];

        let analysis = StorageAnalysis::new();
        for path in file_paths.iter() {
            let path = PathBuf::from(path);
            assert!(analysis.is_irrelevant_file(&path));
        }
    }

}
