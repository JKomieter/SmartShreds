use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::PathBuf;
use sysinfo::System;

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
    pub fn analyse() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        // let dowload_dir = dirs::download_dir().expect("Home directory not found");
        // let desktop_dir = dirs::desktop_dir().expect("Desktop directory not found");
        // let documents_dir = dirs::document_dir().expect("Documents directory not found");
        let home_dir = dirs::home_dir().expect("Home directory not found");

        let total_device_memory = sys.total_memory();
        let memory_used = sys.used_memory();

        let mut analysis = StorageAnalysis {
            total_device_memory,
            memory_usage: MemoryUsage {
                size: memory_used,
                ..Default::default()
            },
            ..Default::default()
        };

        analysis.traverse_directory(&home_dir);

        analysis
    }

    fn traverse_directory(&mut self, start_path: &PathBuf) {
        let mut dir_queue: VecDeque<PathBuf> = VecDeque::new();
        dir_queue.push_back(start_path.to_path_buf());
        let mut unpermitted_dirs: HashSet<PathBuf> = HashSet::new();

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

    fn detect_junk_files(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_storage_analysis() {
        let analysis = StorageAnalysis::analyse();
        assert!(analysis.memory_usage.total_files > 0);
        assert!(analysis.memory_usage.size > 0);
        assert!(analysis.memory_usage.total_folders > 0);
        assert!(analysis.inactive_files.len() > 0);

        println!("{:#?}", analysis);
    }
}
