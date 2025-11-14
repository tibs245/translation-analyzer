use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug))]
pub struct Settings {
    pub common_translations_modules_path: Vec<String>,
    pub translation_file_regex: String,
    pub skip_directories: Vec<String>,
}

#[derive(Error, Debug)]
pub enum SettingsFileManagerError {
    #[error("Unable to read settings file: {0}")]
    UnableToReadPath(String, #[source] std::io::Error),
}

pub fn get_settings(
    settings_file_path: &Path,
) -> Result<Settings, SettingsFileManagerError> {
    match fs::read_to_string(settings_file_path) {
        Ok(invoice_data) => Ok(serde_json::from_str(&invoice_data).unwrap()),
        Err(e) => Err(SettingsFileManagerError::UnableToReadPath(
            settings_file_path.to_string_lossy().to_string(),
            e,
        )),
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            common_translations_modules_path: vec![
                "packages/manager/modules/common-translations".to_string(),
            ],
            translation_file_regex: r#"^Messages_fr_FR\.json$"#.to_string(),
            skip_directories: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                ".idea".to_string(),
                ".vscode".to_string(),
                "dist".to_string(),
                "build".to_string(),
                "manager-tools".to_string(),
            ],
        }
    }
}
