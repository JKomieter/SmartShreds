use crate::shred::{dir_search::DuplicateFile, error::SmartShredsError};

use super::dups_systems::ShredTerminal;

pub fn display_duplicate_files(files: &Vec<DuplicateFile>) -> Result<(), SmartShredsError> {
    let mut shred_terminal = ShredTerminal::init(files.to_vec())?;
    shred_terminal.initiate_display()?;
    shred_terminal.run()?;

    Ok(())
}
