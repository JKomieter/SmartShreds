use std::collections::HashSet;
use std::fs::read_dir;
use std::io::{StdoutLock, Write};
use std::path::PathBuf;
use termion::input::MouseTerminal;
use termion::{color, style};

use super::error::SmartShredsError;

#[derive(Debug)]
pub struct DuplicateFile {
    pub file_name: String,
    pub file_path: String,
}

impl std::fmt::Display for DuplicateFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} in {}", self.file_name, self.file_path)
    }
}

pub fn search_files_with_similar_names_in_dir(
    directory: &PathBuf,
) -> Result<Vec<DuplicateFile>, SmartShredsError> {
    let mut files_set: HashSet<String> = HashSet::new(); // Change this line
    let directory_contents = read_dir(directory)?;
    let mut files: Vec<DuplicateFile> = vec![];

    for entry in directory_contents.into_iter() {
        let entry = entry?;
        let path = entry.path();
        match path.is_dir() {
            true => {
                let mut sub_dir_files = search_files_with_similar_names_in_dir(&path)?;
                files.append(&mut sub_dir_files);
            }
            false => {
                if let Some(file_name) = path.file_name() {
                    let mut file_name_str = file_name.to_string_lossy().to_string();
                    // strip the bracket and the number from the file name
                    let file_extension = path.extension().unwrap_or("txt".as_ref());
                    if let Some(idx) = file_name_str.find(" (") {
                        file_name_str = format!(
                            "{}.{}",
                            &file_name_str[0..idx],
                            file_extension.to_string_lossy()
                        );
                    }
                    if !files_set.insert(file_name_str.clone()) {
                        files.push(DuplicateFile {
                            file_name: file_name_str,
                            file_path: path.to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }
    }
    Ok(files)
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

fn print_duplicates(stdout: &mut StdoutLock, files: &Vec<DuplicateFile>) -> Result<(), SmartShredsError> {
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

pub fn display_duplicate_files(files: Vec<DuplicateFile>) -> Result<(), SmartShredsError> {
    let stdout = MouseTerminal::from(std::io::stdout());
    let mut stdout = stdout.lock();
    if files.is_empty() {
        print_no_duplicates(&mut stdout)?;
    } else {
        print_duplicates(&mut stdout, &files)?;
    }
    Ok(())
}
