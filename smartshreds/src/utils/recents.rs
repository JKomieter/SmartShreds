use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use notify_debouncer_full::{
    new_debouncer,
    notify::{
        event::{CreateKind, DataChange, MetadataKind, ModifyKind, RemoveKind, RenameMode},
        EventKind, RecursiveMode, Watcher,
    },
};

#[derive(Debug)]
pub enum Action {
    Create(String),
    Remove(String),
    Modify(String),
    Move,
    Rename(String),
    Unknown,
}

#[derive(Debug)]
pub enum Node {
    File,
    Folder,
    Unknown,
}

impl From<&PathBuf> for Node {
    fn from(path: &PathBuf) -> Self {
        if path.is_file() {
            Node::File
        } else if path.is_dir() {
            Node::Folder
        } else {
            Node::Unknown
        }
    }
}

impl Into<String> for Node {
    fn into(self) -> String {
        match self {
            Node::File => "file".to_string(),
            Node::Folder => "folder".to_string(),
            Node::Unknown => "".to_string(),
        }
    }
}

impl Action {
    /// The string representation of the action is displayed on the Recents panel.
    pub fn from_event(event_kind: EventKind, node: Node, entry: String) -> Self {
        let node: String = node.into();

        match event_kind {
            EventKind::Create(kind) => match kind {
                CreateKind::File => {
                    Action::Create(format!("The {} {} has been created", node, entry))
                }
                CreateKind::Folder => {
                    Action::Create(format!("The {} {} has been created", node, entry))
                }
                _ => Action::Create(format!("{}{} has been created", node, entry)),
            },
            EventKind::Remove(kind) => match kind {
                RemoveKind::File => {
                    Action::Remove(format!("The {} {} has been removed", node, entry))
                }
                RemoveKind::Folder => {
                    Action::Remove(format!("The {} {} has been removed", node, entry))
                }
                _ => Action::Remove(format!("{}{} has been removed", node, entry)),
            },
            EventKind::Modify(kind) => match kind {
                ModifyKind::Data(data_changed) => match data_changed {
                    DataChange::Content => Action::Modify(format!(
                        "The content of the {} {} has been modified",
                        node, entry
                    )),
                    DataChange::Size => Action::Modify(format!(
                        "The size of the {} {} has been modified",
                        node, entry
                    )),
                    _ => Action::Modify(format!("The {} {} has been modified", node, entry)),
                },
                ModifyKind::Name(rename_mode) => match rename_mode {
                    RenameMode::From => {
                        Action::Rename(format!("The {} {} has been renamed", node, entry))
                    }
                    RenameMode::To => {
                        Action::Rename(format!("The {} {} has been renamed", node, entry))
                    }
                    _ => Action::Rename(format!("The {} {} has been renamed", node, entry)),
                },
                ModifyKind::Metadata(metadata_kind) => match metadata_kind {
                    MetadataKind::Permissions => Action::Modify(format!(
                        "The permissions of the {} {} have been modified",
                        node, entry
                    )),
                    MetadataKind::Ownership => Action::Modify(format!(
                        "The ownership of the {} {} has been modified",
                        node, entry
                    )),
                    MetadataKind::WriteTime => Action::Modify(format!(
                        "The write time of the {} {} has been modified",
                        node, entry
                    )),
                    MetadataKind::AccessTime => Action::Modify(format!(
                        "The access time of the {} {} has been modified",
                        node, entry
                    )),
                    MetadataKind::Extended => Action::Modify(format!(
                        "The extended metadata of the {} {} has been modified",
                        node, entry
                    )),
                    _ => Action::Modify(format!("The {} {} has been modified", node, entry)),
                },
                _ => Action::Modify(format!("The {} {} has been modified", node, entry)),
            },
            EventKind::Access(_) => {
                Action::Modify(format!("The {} {} has been accessed", node, entry))
            }
            _ => Action::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct ActionEvent {
    pub action: Action,
    pub file_path: PathBuf,
}

pub fn watch(path: &Path) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer =
        new_debouncer(Duration::from_secs(2), None, tx).expect("Failed to create debouncer");

    debouncer.watcher().watch(path, RecursiveMode::Recursive).expect("Failed to watch path");

    for res in rx {
        match res {
            Ok(events) => {
                events.iter().for_each(|event| {
                    println!("Original event: {:?}", event);
                    let event_kind = event.kind;
                    let path = event.paths.first().expect("No path found");
                    let entry = path
                        .file_name()
                        .expect("No file/folder name found")
                        .to_str()
                        .unwrap()
                        .to_string();
                    let node = Node::from(path);
                    let action = Action::from_event(event_kind, node, entry);

                    let action_event = ActionEvent {
                        action,
                        file_path: path.to_path_buf(),
                    };

                    println!("{:?}", action_event);
                });
            }
            Err(error) => println!("Error: {error:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    
}
