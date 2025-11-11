use std::io::Error;
use std::path::{Path, PathBuf};
use thiserror::Error;
use regex::Regex;
use std::fs;
use std::sync::Arc;
use rayon::prelude::*;

#[derive(Error, Debug)]
pub enum SearchAllTranslationsFilesError {
    // #[error("Project path not found: {0}")]
    // ProjectPathNotFound(String),

    #[error("Unable to read path: {0}")]
    UnableToReadPath(String, #[source] Error),

    #[error("Invalid regex pattern: {0} - {1}")]
    InvalidRegexPattern(String, String),
}

/// Recursively searches for regex matches in all files within a path
/// Returns a vector of tuples: (file_path, line_number, matched_text)
pub fn search_recursive_regex(
    root_path: &Path,
    regex_pattern: &str,
    paths_to_skip: &[String],
) -> Result<Vec<Box<PathBuf>>, SearchAllTranslationsFilesError> {
    let regex = Regex::new(regex_pattern)
        .map_err(|e| SearchAllTranslationsFilesError::InvalidRegexPattern(regex_pattern.to_string(), e.to_string()))?;

    let regex = Arc::new(regex);
    let results = Arc::new(parking_lot::Mutex::new(Vec::new()));

    search_recursive_parallel(root_path, regex, paths_to_skip, results.clone())?;

    let final_results = results.lock().clone();

    Ok(final_results)
}

fn search_recursive_parallel(
    path: &Path,
    regex: Arc<Regex>,
    paths_to_skip: &[String],
    results: Arc<parking_lot::Mutex<Vec<Box<PathBuf>>>>,
) -> Result<(), SearchAllTranslationsFilesError> {
    let entries = fs::read_dir(path)
        .map_err(|e| SearchAllTranslationsFilesError::UnableToReadPath(
            path.to_string_lossy().to_string(),
            e,
        ))?;

    let paths: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    paths.par_iter().for_each(|entry_path| {
        process_entry(&entry_path, regex.clone(), paths_to_skip, results.clone()).expect(&format!("Unable to process: {}", entry_path.to_string_lossy()));
    });

    Ok(())
}

fn process_entry(
    path: &Path,
    regex: Arc<Regex>,
    paths_to_skip: &[String],
    results: Arc<parking_lot::Mutex<Vec<Box<PathBuf>>>>
) -> Result<(), SearchAllTranslationsFilesError> {
    if path.is_dir() {
        // Skip hidden directories and common non-source directories
        if should_skip_directory(path, paths_to_skip) {
            return Ok(());
        }
        search_recursive_parallel(path, regex, paths_to_skip, results)?;
    } else if path.is_file() && regex.is_match(path.file_name().unwrap().to_string_lossy().as_ref()) {
        results.lock().push(Box::new(path.to_owned()))
    }
    Ok(())
}

fn should_skip_directory(path: &Path, paths_to_skip: &[String]) -> bool {
    if let Some(file_name) = path.file_name() {
        let name = file_name.to_string_lossy();
        paths_to_skip.contains(&name.to_string())
    } else {
        false
    }
}
