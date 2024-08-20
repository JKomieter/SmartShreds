use adw::prelude::*;
use gtk::{cairo, gio, DrawingArea, Image, MediaFile, TextView, Video, Widget};
use rsvg;
use std::{fs::File, io::Read, path::PathBuf};

pub enum PreviewFileType {
    Image,
    Svg,
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
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "ico" | "tiff" | "tif" | "svgz"
            | "eps" | "psd" | "ai" | "xcf" | "psb" | "pdd" => Self::Image,
            "mp3" | "flac" | "wav" | "ogg" | "m4a" => Self::Audio,
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rs" | "md"
            | "html" | "xml" | "json" | "csv" | "log" | "conf" | "ini" | "yml" | "yaml"
            | "toml" | "sh" | "bat" | "ps1" | "py" | "js" | "ts" | "c" | "cpp" | "h" | "hpp"
            | "cs" | "java" | "kt" | "swift" | "rb" | "pl" | "php" | "go" | "sql" | "asm"
            | "asmx" | "aspx" | "jsp" | "cshtml" | "jsx" | "tsx" | "rsx" | "vue" | "svelte"
            | "elm" | "ml" | "fs" | "fsx" | "fsi" | "clj" | "cljs" | "cljc" | "edn" | "ex"
            | "pak" | "exs" | "erl" | "hrl" | "hs" | "lhs" | "purs" | "scm" | "ss" | "rkt"
            | "jl" | "dylib" => Self::Document,
            "svg" => Self::Svg,
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
                let image = Image::from_file(&self.path);
                Some(image.upcast())
            }
            PreviewFileType::Video => {
                let file = gio::File::for_path(self.path.clone());
                let media_stream = MediaFile::for_file(&file);
                media_stream.play();
                let video = Video::builder().media_stream(&media_stream).build();
                video.display();
                Some(video.upcast())
            }
            PreviewFileType::Audio => None,
            PreviewFileType::Document => {
                if let Ok(mut file) = File::open(&self.path) {
                    let mut buffer = Vec::new();
                    let _ = file.read_to_end(&mut buffer);
                    let text = String::from_utf8_lossy(&buffer);
                    let textview = TextView::builder()
                        .editable(false)
                        .vscroll_policy(gtk::ScrollablePolicy::Natural)
                        .build();
                    textview.buffer().set_text(&text);
                    Some(textview.upcast())
                } else {
                    let label = gtk::Label::new(Some("Failed to open file"));
                    label.set_halign(gtk::Align::Center);
                    label.set_valign(gtk::Align::Center);
                    Some(label.upcast())
                }
            }
            PreviewFileType::Svg => {
                // let path = self.path.clone();
                // let drawing_area = DrawingArea::builder().build();
                // drawing_area.set_draw_func(move |drawing_area, cr, width, height| {
                //     let handle = rsvg::Loader::new()
                //         .read_path(path.clone())
                //         .expect("Failed to read svg file");
                //     let surface =
                //         cairo::ImageSurface::create(cairo::Format::ARgb32, width, height).unwrap();
                //     let cr =
                //         cairo::Context::new(&surface).expect("Failed to create a cairo context");
                //     let renderer = rsvg::CairoRenderer::new(&handle);
                    
                // });

                // Some(drawing_area.upcast())
                let label = gtk::Label::new(Some("SVG preview is not supported yet"));
                label.set_halign(gtk::Align::Center);
                label.set_valign(gtk::Align::Center);
                Some(label.upcast())
            }
            PreviewFileType::Other => {
                let label = gtk::Label::new(Some("Unsupported file type"));
                label.set_halign(gtk::Align::Center);
                label.set_valign(gtk::Align::Center);
                Some(label.upcast())
            }
        }
    }
}
