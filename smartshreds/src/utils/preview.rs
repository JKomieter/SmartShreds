use adw::prelude::*;
use gtk::{
    glib::{GString, Object}, Image, TextView, Video, Widget
};
use std::{fs::File, io::Read, path::PathBuf};

pub enum PreviewFileType {
    Image,
    Video,
    Audio,
    Document,
    Other,
}

impl Default for PreviewFileType {
    fn default() -> Self {
        Self::Other
    }
}

impl From<&str> for PreviewFileType {
    fn from(value: &str) -> Self {
        match value {
            "mp4" | "mkv" | "avi" | "webm" | "mov" => Self::Video,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "ico" | "tiff" | "tif" | "svg"
            | "svgz" | "eps" | "psd" | "ai" | "xcf" | "psb" | "pdd" => Self::Image,
            "mp3" | "flac" | "wav" | "ogg" | "m4a" => Self::Audio,
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rs" | "md"
            | "html" | "xml" | "json" | "csv" | "log" | "conf" | "ini" | "yml" | "yaml"
            | "toml" | "sh" | "bat" | "ps1" | "py" | "js" | "ts" | "c" | "cpp" | "h" | "hpp"
            | "cs" | "java" | "kt" | "swift" | "rb" | "pl" | "php" | "go" | "sql" | "asm"
            | "asmx" | "aspx" | "jsp" | "cshtml" | "jsx" | "tsx" | "rsx" | "vue" | "svelte"
            | "elm" | "ml" | "fs" | "fsx" | "fsi" | "clj" | "cljs" | "cljc" | "edn" | "ex"
            | "exs" | "erl" | "hrl" | "hs" | "lhs" | "purs" | "scm" | "ss" | "rkt" | "jl" => {
                Self::Document
            }
            _ => Self::Other,
        }
    }
}

#[derive(Default)]
pub struct Preview {
    pub path: PathBuf,
    pub file_type: PreviewFileType,
}

impl Preview {
    pub fn new(path: PathBuf) -> Self {
        let file_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => PreviewFileType::from(ext.to_lowercase().as_str()),
            None => PreviewFileType::Document,
        };

        Self { path, file_type }
    }

    pub fn widget(&self) -> Option<Widget> {
        match self.file_type {
            PreviewFileType::Image => {
                let path = GString::from(self.path.to_string_lossy().as_ref());
                let image = Image::builder()
                    .file(path)
                    .vexpand(true)
                    .hexpand(true)
                    .build();
                Some(image.upcast())
            }
            PreviewFileType::Video => {
                let video = Video::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .build();
                video.set_filename(Some(self.path.to_string_lossy().as_ref()));
                video.set_autoplay(true);
                Some(video.upcast())
            },
            PreviewFileType::Audio => None,
            PreviewFileType::Document => {
                if let Ok(mut file) = File::open(&self.path) {
                    let mut buffer = Vec::new();
                    let _ = file.read_to_end(&mut buffer);
                    let text = String::from_utf8_lossy(&buffer);
                    let textview = TextView::builder().build();
                    textview.buffer().set_text(&text);
                    Some(textview.upcast())
                } else {
                    let label = gtk::Label::new(Some("Failed to open file"));
                    label.set_halign(gtk::Align::Center);
                    label.set_valign(gtk::Align::Center);
                    Some(label.upcast())
                }
            }
            PreviewFileType::Other => None,
        }
    }
}
