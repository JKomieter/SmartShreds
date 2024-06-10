// use std::path::PathBuf;

// #[derive(Debug, PartialEq, Eq)]
// pub enum FileType {
//     Image,
//     Video,
//     Audio,
//     Document,
//     Code,
//     Archive,
//     Other,
// }

// impl FileType {
//     pub fn type_from_extension(path: &PathBuf) -> Self {
//         let Some(extension) = Self::file_extension_from_path(&path) else {
//             return Self::Other;
//         };
//         match extension.to_lowercase().as_str() {
//             "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "tiff" | "ico" => Self::Image,
//             "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "vob" | "3gp" | "mpg" | "mpeg" | "m4v" | "m2ts" | "ts" | "mts" => Self::Video,
//             "mp3" | "wav" | "flac" | "ogg" | "m4a" | "wma" | "aac" | "aiff" => Self::Audio,
//             "doc" | "docx" | "pdf" | "txt" | "rtf" | "odt" | "ods" | "odp" | "xls" | "xlsx" | "ppt" | "pptx" => Self::Document,
//             "html" | "css" | "js" | "ts" | "rs" | "py" | "java" | "c" | "cpp" | "h" | "hpp" | "go" | "rb" | "php" | "pl" | "sh" | "bat" | "ps1" | "vbs" | "sql" | "json" | "xml" => Self::Code,
//             "zip" | "rar" | "7z" | "tar" | "gz" | "xz" | "bz2" | "zst" | "lz" | "lz4" | "lzh" | "lha" | "arj" | "cab" | "iso" | "img" => Self::Archive,
//             _ => Self::Other,
//         }
//     }

//     fn file_extension_from_path(path: &PathBuf) -> Option<&str> {
//         path.extension()?.to_str()
//     }
// }

