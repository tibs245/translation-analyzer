pub(crate) use crate::entities::Translation;
use rayon::prelude::*;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadTranslationsFilesError {
    #[error("Unable to read or parse JSON format: {0}")]
    UnableReadFormat(String),

    #[error("Failed to read file: {0}")]
    FileReadError(String, #[source] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(String, #[source] serde_json::error::Error),
}

/// Recursively searches for regex matches in all files within a path
/// Returns a vector of tuples: (file_path, line_number, matched_text)
pub fn load_translations(
    translation_files_path: Vec<Box<PathBuf>>,
) -> Result<Vec<Translation>, LoadTranslationsFilesError> {
    let results = Arc::new(parking_lot::Mutex::new(Vec::new()));

    load_translations_parallel(translation_files_path, results.clone())?;

    let final_results = results.lock().clone();

    Ok(final_results)
}

fn load_translations_parallel(
    translation_files_path: Vec<Box<PathBuf>>,
    results: Arc<parking_lot::Mutex<Vec<Translation>>>,
) -> Result<(), LoadTranslationsFilesError> {
    translation_files_path.par_iter().for_each(|entry_path| {
        load_translation_file(&entry_path, results.clone()).expect(&format!(
            "Unable to process: {}",
            entry_path.to_string_lossy()
        ));
    });

    Ok(())
}

fn load_translation_file(
    path: &Path,
    results: Arc<parking_lot::Mutex<Vec<Translation>>>,
) -> Result<(), LoadTranslationsFilesError> {
    // Verify the file has .json extension
    if path.extension().and_then(|s| s.to_str()) != Some("json") {
        return Err(LoadTranslationsFilesError::UnableReadFormat(format!(
            "File is not a JSON file: {}",
            path.display()
        )));
    }

    // Read the file content
    let content = fs::read_to_string(path).map_err(|e| {
        LoadTranslationsFilesError::FileReadError(
            format!("Cannot read file: {}", path.display()),
            e,
        )
    })?;

    // Parse JSON into Map<String, String>
    let json_value: Value = serde_json::from_str(&content).map_err(|e| {
        LoadTranslationsFilesError::JsonError(
            format!("Invalid JSON format in {}", path.display()),
            e,
        )
    })?;

    // Extract the object and convert to Vec<Translation>
    if let Value::Object(map) = json_value {
        let translations: Vec<Translation> = map
            .into_iter()
            .map(|(key, value)| Translation {
                path: path.to_path_buf(),
                translations: value.to_string(),
                key,
            })
            .collect();

        // Extend results with the new translations
        let mut results_lock = results.lock();
        results_lock.extend(translations);
    } else {
        return Err(LoadTranslationsFilesError::UnableReadFormat(format!(
            "Root element is not a JSON object: {}",
            path.display()
        )));
    }

    Ok(())
}
