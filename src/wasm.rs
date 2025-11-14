use serde::{Deserialize, Serialize};
use std::path::Path;
use wasm_bindgen::prelude::*;

use crate::analyse_project_duplication::{DuplicationType, analyse_duplication};
use crate::get_translation_for_project::get_translations_for_project;
use crate::load_translations::load_translations;
use crate::map_translations_by_key::map_translations_by_translation;
use crate::map_translations_by_project::map_translations_by_project;
use crate::search_recursive_regex::search_recursive_regex;
use crate::settings::Settings;

// Initialize panic hook for better error messages in WASM
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct AnalyzerOptions {
    translation_file_regex: String,
    skip_directories: Vec<String>,
    common_translations_modules_path: Vec<String>,
}

#[wasm_bindgen]
impl AnalyzerOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AnalyzerOptions {
        let default_settings = Settings::default();
        AnalyzerOptions {
            translation_file_regex: default_settings.translation_file_regex,
            skip_directories: default_settings.skip_directories,
            common_translations_modules_path: default_settings
                .common_translations_modules_path,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn translation_file_regex(&self) -> String {
        self.translation_file_regex.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_translation_file_regex(&mut self, value: String) {
        self.translation_file_regex = value;
    }

    #[wasm_bindgen(getter)]
    pub fn skip_directories(&self) -> Vec<String> {
        self.skip_directories.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_skip_directories(&mut self, value: Vec<String>) {
        self.skip_directories = value;
    }

    #[wasm_bindgen(getter)]
    pub fn common_translations_modules_path(&self) -> Vec<String> {
        self.common_translations_modules_path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_common_translations_modules_path(&mut self, value: Vec<String>) {
        self.common_translations_modules_path = value;
    }
}

impl Default for AnalyzerOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DuplicationReportData {
    pub translation_key: String,
    pub translation_value: String,
    pub file_path: String,
    pub duplication_type: String,
    pub occurrences_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalReportResult {
    pub files_found: usize,
    pub inter_package_duplication: usize,
    pub common_translation_duplication: usize,
    pub external_projects_duplication: usize,
    pub total_duplication: usize,
}

#[derive(Serialize, Deserialize)]
pub struct DetailedReportResult {
    pub files_found: usize,
    pub global_report: GlobalReportResult,
    pub duplications: Vec<DuplicationReportData>,
}

/// Get a global duplication report for a specific project
#[wasm_bindgen]
pub fn get_global_report_for_project(
    monorepo_path: &str,
    package_path: &str,
    options: &AnalyzerOptions,
) -> Result<JsValue, JsValue> {
    let settings = Settings {
        common_translations_modules_path: options
            .common_translations_modules_path
            .clone(),
        translation_file_regex: options.translation_file_regex.clone(),
        skip_directories: options.skip_directories.clone(),
    };

    let path = Path::new(monorepo_path);

    let matches = search_recursive_regex(
        path,
        &settings.translation_file_regex,
        &settings.skip_directories,
    )
    .map_err(|e| JsValue::from_str(&format!("Failed to search files: {}", e)))?;

    let files_found = matches.len();

    let translations = load_translations(matches)
        .map_err(|e| JsValue::from_str(&format!("Failed to load translations: {}", e)))?;

    let translations_indexed = map_translations_by_translation(&translations);
    let project_translations = get_translations_for_project(package_path, &translations);

    let reports_duplication =
        analyse_duplication(&package_path, &project_translations, &translations_indexed);

    let inter_package_duplication = reports_duplication
        .iter()
        .filter(|d| d.duplication_type == DuplicationType::InterPackage)
        .count();

    let common_translation_duplication = reports_duplication
        .iter()
        .filter(|d| d.duplication_type == DuplicationType::CommonTranslation)
        .count();

    let external_projects_duplication = reports_duplication
        .iter()
        .filter(|d| d.duplication_type == DuplicationType::ExternalProjects)
        .count();

    let result = GlobalReportResult {
        files_found,
        inter_package_duplication,
        common_translation_duplication,
        external_projects_duplication,
        total_duplication: inter_package_duplication
            + common_translation_duplication
            + external_projects_duplication,
    };

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get a detailed duplication report for a specific project
#[wasm_bindgen]
pub fn get_detailed_report_for_project(
    monorepo_path: &str,
    package_path: &str,
    options: &AnalyzerOptions,
) -> Result<JsValue, JsValue> {
    let settings = Settings {
        common_translations_modules_path: options
            .common_translations_modules_path
            .clone(),
        translation_file_regex: options.translation_file_regex.clone(),
        skip_directories: options.skip_directories.clone(),
    };

    let path = Path::new(monorepo_path);

    let matches = search_recursive_regex(
        path,
        &settings.translation_file_regex,
        &settings.skip_directories,
    )
    .map_err(|e| JsValue::from_str(&format!("Failed to search files: {}", e)))?;

    let files_found = matches.len();

    let translations = load_translations(matches)
        .map_err(|e| JsValue::from_str(&format!("Failed to load translations: {}", e)))?;

    let translations_indexed = map_translations_by_translation(&translations);
    let project_translations = get_translations_for_project(package_path, &translations);

    let reports_duplication =
        analyse_duplication(&package_path, &project_translations, &translations_indexed);

    // Calculate global stats
    let inter_package_duplication = reports_duplication
        .iter()
        .filter(|d| d.duplication_type == DuplicationType::InterPackage)
        .count();

    let common_translation_duplication = reports_duplication
        .iter()
        .filter(|d| d.duplication_type == DuplicationType::CommonTranslation)
        .count();

    let external_projects_duplication = reports_duplication
        .iter()
        .filter(|d| d.duplication_type == DuplicationType::ExternalProjects)
        .count();

    // Build detailed duplication data
    let mut duplications = Vec::new();
    let mut seen_translations = std::collections::HashSet::new();

    for duplication in &reports_duplication {
        let translation_value = duplication.translation.translations.clone();

        if seen_translations.contains(&translation_value) {
            continue;
        }
        seen_translations.insert(translation_value.clone());

        let other_usages = translations_indexed.get(&translation_value).unwrap();

        let duplication_data = DuplicationReportData {
            translation_key: duplication.translation.key.clone(),
            translation_value: translation_value.clone(),
            file_path: duplication.translation.path.to_string_lossy().to_string(),
            duplication_type: format!("{:?}", duplication.duplication_type),
            occurrences_count: other_usages.len(),
        };

        duplications.push(duplication_data);
    }

    let result = DetailedReportResult {
        files_found,
        global_report: GlobalReportResult {
            files_found,
            inter_package_duplication,
            common_translation_duplication,
            external_projects_duplication,
            total_duplication: inter_package_duplication
                + common_translation_duplication
                + external_projects_duplication,
        },
        duplications,
    };

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Get a global report for all projects in the monorepo
#[wasm_bindgen]
pub fn get_global_report_all(
    monorepo_path: &str,
    options: &AnalyzerOptions,
) -> Result<JsValue, JsValue> {
    let settings = Settings {
        common_translations_modules_path: options
            .common_translations_modules_path
            .clone(),
        translation_file_regex: options.translation_file_regex.clone(),
        skip_directories: options.skip_directories.clone(),
    };

    let path = Path::new(monorepo_path);

    let matches = search_recursive_regex(
        path,
        &settings.translation_file_regex,
        &settings.skip_directories,
    )
    .map_err(|e| JsValue::from_str(&format!("Failed to search files: {}", e)))?;

    let files_found = matches.len();

    let translations = load_translations(matches)
        .map_err(|e| JsValue::from_str(&format!("Failed to load translations: {}", e)))?;

    let translations_indexed = map_translations_by_translation(&translations);
    let mapped_by_project = map_translations_by_project(&translations);

    let mut all_reports = Vec::new();

    for (package_path, project_translations) in &mapped_by_project {
        let reports = analyse_duplication(
            package_path,
            project_translations,
            &translations_indexed,
        );

        let inter_package = reports
            .iter()
            .filter(|d| d.duplication_type == DuplicationType::InterPackage)
            .count();

        let common_translation = reports
            .iter()
            .filter(|d| d.duplication_type == DuplicationType::CommonTranslation)
            .count();

        let external_projects = reports
            .iter()
            .filter(|d| d.duplication_type == DuplicationType::ExternalProjects)
            .count();

        all_reports.push(serde_json::json!({
            "package_path": package_path,
            "files_found": files_found,
            "inter_package_duplication": inter_package,
            "common_translation_duplication": common_translation,
            "external_projects_duplication": external_projects,
            "total_duplication": inter_package + common_translation + external_projects,
        }));
    }

    let result = serde_json::json!({
        "files_found": files_found,
        "projects": all_reports,
    });

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}
