mod shred;
use std::path::PathBuf;

use structopt::StructOpt;
use shred::{dir_search, error::SmartShredsError};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "smartshreds", 
    about = "Smartshreds is file duplication detector with semantic similarity analysis",
    help = "Run `cargo run -- --directory-path /path/to/directory/to/scan/for/duplications`"
)]
struct CommandLine {
    #[structopt(short, long, help = "Specify the directory path to scan for file duplication.")]
    pub directory_path: PathBuf
}


fn main() -> Result<(), SmartShredsError>{
    let args: CommandLine = CommandLine::from_args();
    let directory_path = &args.directory_path;
    let dup_files = dir_search::search_files_with_similar_names_in_dir(directory_path)?;
    let _ = dir_search::display_duplicate_files(dup_files);
    
    Ok(())
}
