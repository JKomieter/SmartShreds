use regex::Regex;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::io::{StdoutLock, Write};
use std::path::PathBuf;
use termion::input::MouseTerminal;
use termion::{color, style};

use super::error::SmartShredsError;

#[derive(Debug)]
pub struct DuplicateFile {
    pub file_name: String,
    pub file_paths: Vec<PathBuf>,
}

impl std::fmt::Display for DuplicateFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} has duplicates: {}",
            self.file_name,
            self.file_paths
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub fn search_files_with_similar_names_in_dir(
    directory: &PathBuf,
) -> Result<Vec<DuplicateFile>, SmartShredsError> {
    let mut dir_queue: Vec<PathBuf> = vec![directory.clone()];
    let mut duplicates: HashMap<String, Vec<PathBuf>> = HashMap::new();

    while let Some(dir) = dir_queue.pop() {
        for entry in read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dir_queue.push(path);
            } else {
                // strip if there is (1) or (2) etc. in the file name
                let file_name = path
                    .file_name()
                    .unwrap_or_else(|| OsStr::new(""))
                    .to_string_lossy()
                    .to_string();
                let re = Regex::new(r"\s\([0-9]\)").unwrap();
                let original_file_name = re.replace_all(&file_name, "").into_owned();
                if duplicates.contains_key(&original_file_name) {
                    duplicates
                        .get_mut(&original_file_name)
                        .expect("Could not get duplucates")
                        .push(path.clone());
                } else {
                    println!("{}", file_name);
                    duplicates
                        .entry(original_file_name)
                        .or_insert_with(|| vec![path.clone()]);
                }
            }
        }
    }

    let duplicate_files = duplicates
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .map(|(file_name, file_paths)| DuplicateFile {
            file_name: file_name.to_string(),
            file_paths: file_paths.clone(),
        })
        .collect();

    Ok(duplicate_files)
}

fn print_no_duplicates(stdout: &mut StdoutLock) -> Result<(), SmartShredsError> {
    writeln!(
        stdout,
        "{}{}No duplicate files found in the directory.{}",
        style::Bold,
        color::Fg(color::Green),
        style::Reset
    )?;
    Ok(())
}

fn print_duplicates(
    stdout: &mut StdoutLock,
    files: &Vec<DuplicateFile>,
) -> Result<(), SmartShredsError> {
    writeln!(
        stdout,
        "{}{}Duplicate files found in the directory:{}",
        style::Bold,
        color::Fg(color::Yellow),
        style::Reset
    )?;

    for (index, file) in files.iter().enumerate() {
        writeln!(
            stdout,
            "{}{}{} - {}{}",
            style::Bold,
            color::Fg(color::Red),
            index + 1,
            file,
            style::Reset
        )?;
    }

    Ok(())
}

pub fn display_duplicate_files(files: &Vec<DuplicateFile>) -> Result<(), SmartShredsError> {
    let stdout = MouseTerminal::from(std::io::stdout());
    let mut stdout = stdout.lock();
    if files.is_empty() {
        print_no_duplicates(&mut stdout)?;
    } else {
        print_duplicates(&mut stdout, &files)?;
    }
    Ok(())
}
