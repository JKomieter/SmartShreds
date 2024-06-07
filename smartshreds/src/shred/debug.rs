use std::io::{StdoutLock, Write};

use termion::{color, input::MouseTerminal, style};

use super::{dir_search::DuplicateFile, error::SmartShredsError};

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
