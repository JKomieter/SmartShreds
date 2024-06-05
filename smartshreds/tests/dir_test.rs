use std::path::PathBuf;

use smartshreds::shred::dir_search::search_files_with_similar_names_in_dir;


#[test]
fn test_search_files_with_similar_names_in_dir() {
    let directory = PathBuf::from("dir");
    let result = search_files_with_similar_names_in_dir(&directory);
    assert!(result.is_ok());
    let files = result.unwrap();
    assert_eq!(files.len(), 1);
    let file = files.get(0).unwrap();
    assert_eq!(file.file_name, "a.txt");
}
