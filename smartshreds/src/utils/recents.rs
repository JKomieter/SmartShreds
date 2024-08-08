use std::path::Path;
use notify::{Config, PollWatcher, RecursiveMode, Watcher};

#[derive(Debug)]
pub enum Actions {
    Add,
    Remove,
    Update,
    Move,
    Rename,
}


fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = PollWatcher::new(tx, Config::default().with_manual_polling())?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
            }
            Err(e) => {
                eprintln!("Error receiving event: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_watch() {
        let path = dirs::download_dir().unwrap();
        let res = watch(path);
        
        assert!(res.is_ok());
   
    }
}