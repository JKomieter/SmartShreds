use std::path::PathBuf;

use regex::RegexSet;
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

#[test]
fn test_regex() {
    let set = RegexSet::new(&[
        r"\w+", r"\d+", r"\pL+", r"foo", r"bar", r"barfoo", r"foobar",
    ])
    .unwrap();

    // Iterate over and collect all of the matches. Each match corresponds to the
    // ID of the matching pattern.
    let matches: Vec<_> = set.matches("foobar").into_iter().collect();
    assert_eq!(matches, vec![0, 2, 3, 4, 6]);

    // You can also test whether a particular regex matched:
    let matches = set.matches("foobar");
    assert!(!matches.matched(5));
    assert!(matches.matched(6));
}
