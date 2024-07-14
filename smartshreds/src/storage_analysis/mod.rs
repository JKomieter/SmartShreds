use std::fs;
use std::path::PathBuf;
use sysinfo::System;
use chrono::{DateTime, Utc};
use std::time::SystemTime;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct StorageAnalysis {
    pub file_types_info: Vec<FileTypesDetails>,
    pub inactive_files: Vec<InActiveFiles>,
    pub junk_files: Vec<JunkFiles>, 
    pub total_number_of_files: u64,
    pub memory_usage: MemoryUsage,
    pub total_device_memory: u64,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryUsage {
    pub size: u64,
    pub total_files: u64,
    pub total_folders: u64,
}

#[derive(Debug, Clone, Default)]
pub struct FileTypesDetails {
    pub file_type: FileType,
    pub number_of_files: u64,
    pub space_used: u64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileType {
    Image,
    Video,
    Audio,
    Document,
    Code,
    Other
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Other
    }
}   

#[derive(Debug, Clone, Default)]
pub struct InActiveFiles {
    pub file_location: PathBuf,
    pub last_accessed: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub size: u64,
    pub file_type: FileType,
}

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
            "js"  | "html" | "css" | "py" | "go" | "java" | "cpp" | "c" | "h" | "hpp" | "rs" | "ts" | "json" | "xml" | "yaml" | "toml" => FileType::Code,
            _ => FileType::Other
        }
    }
}

impl StorageAnalysis {
    pub fn analyse(path: &PathBuf) -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_device_memory = sys.total_memory();
        let memory_used = sys.used_memory();
        
        let mut analysis = StorageAnalysis {
            total_device_memory,
            memory_usage: MemoryUsage { size: memory_used, ..Default::default() },
            ..Default::default()
        };
        
        analysis.traverse_directory(path);
        analysis
    }

    fn traverse_directory(&mut self, path: &PathBuf) {
        let mut file_types_map: HashMap<FileType, (u64, u64)> = HashMap::new();
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                if path.is_dir() {
                    self.memory_usage.total_folders += 1;
                    self.traverse_directory(&path);
                } else {
                    self.memory_usage.total_files += 1;
                    self.total_number_of_files += 1;
                    self.process_file(&path, &mut file_types_map);
                }
            }
        }
        
        for (file_type, (count, space)) in file_types_map {
            self.file_types_info.push(FileTypesDetails {
                file_type,
                number_of_files: count,
                space_used: space,
            });
        }
    }

    fn process_file(&mut self, path: &PathBuf, file_types_map: &mut HashMap<FileType, (u64, u64)>) {
        if let Ok(metadata) = fs::metadata(path) {
            let file_size = metadata.len();
            let file_extension = path.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("");
            let file_type: FileType = file_extension.into();

            let entry = file_types_map.entry(file_type.clone()).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += file_size;

            if let Ok(accessed) = metadata.accessed().map(|t| DateTime::<Utc>::from(t)) {
                if accessed < (Utc::now() - chrono::Duration::days(365)) {
                    self.inactive_files.push(InActiveFiles {
                        file_location: path.clone(),
                        last_accessed: accessed,
                        last_modified: DateTime::<Utc>::from(metadata.modified().unwrap_or(SystemTime::now())),
                        size: file_size,
                        file_type: file_type.clone(),
                    });
                }
            }

            self.detect_junk_files(path, &file_type);
        }
    }

    fn detect_junk_files(&mut self, path: &PathBuf, file_type: &FileType) {
        if path.extension().and_then(std::ffi::OsStr::to_str) == Some("log") {
            self.junk_files.push(JunkFiles::Logs);
        }
        // Add more conditions for other junk file types as needed
    }
}
