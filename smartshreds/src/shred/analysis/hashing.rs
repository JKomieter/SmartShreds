use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use super::super::dir_search::DuplicateFile;
use super::super::error::SmartShredsError;

#[derive(Debug)]
pub struct HashSimlarity {
    pub original_file_name: String,
    pub hashes: HashMap<String, Vec<PathBuf>>
}

pub fn hash_duplicate_file(duplicates: &Vec<DuplicateFile>) -> Result<Vec<HashSimlarity>, SmartShredsError> {
    let mut hasher = Sha256::new();
    let mut hash_similarities = Vec::new();

    for duplicate in duplicates {
        let mut hash_similarity = HashSimlarity {
            original_file_name: duplicate.file_name.clone(),
            hashes: HashMap::new()
        };
        for file_path in &duplicate.file_paths {
            let mut file = fs::File::open(file_path)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            hasher.update(&contents);
            let result = hasher.finalize_reset();
            let hash = format!("{:x}", result);

            hash_similarity.hashes.entry(hash).or_insert_with(Vec::new).push(file_path.clone());
        }
        hash_similarities.push(hash_similarity);
    }

    Ok(hash_similarities)
}
