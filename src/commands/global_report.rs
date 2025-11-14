use crate::analyse_project_duplication::{
    analyse_duplication, print_global_duplication_report,
};
use crate::get_translation_for_project::get_translations_for_project;
use crate::load_translations::load_translations;
use crate::map_translations_by_key::map_translations_by_translation;
use crate::map_translations_by_project::map_translations_by_project;
use crate::search_recursive_regex::search_recursive_regex;
use crate::settings::Settings;
use std::error::Error;
use std::path::Path;

/// Generate global report for all projects in the monorepo
pub fn global_report_all(
    monorepo_path: &Path,
    config: Settings,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    let matches = search_recursive_regex(
        monorepo_path,
        &config.translation_file_regex,
        &config.skip_directories,
    )
    .unwrap();
    println!("Found {} files", matches.len());

    let translations = load_translations(matches).expect("Cannot map translations");

    let translations_indexed = map_translations_by_translation(&translations);

    let mapped_by_project = map_translations_by_project(&translations);

    for package_path in mapped_by_project.keys() {
        println!("Analyse project : {}", package_path);
        let reports_duplication = analyse_duplication(
            &package_path,
            &mapped_by_project[package_path],
            &translations_indexed,
        );
        print_global_duplication_report(&reports_duplication);
    }

    Ok(())
}

/// Generate global report for a specific project
pub fn global_report_for_project(
    monorepo_path: &Path,
    config: Settings,
    package_path: &str,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    let matches = search_recursive_regex(
        monorepo_path,
        &config.translation_file_regex,
        &config.skip_directories,
    )
    .unwrap();
    println!("Found {} files", matches.len());

    let translations = load_translations(matches).expect("Cannot map translations");

    let translations_indexed = map_translations_by_translation(&translations);

    let project_translations = get_translations_for_project(package_path, &translations);

    println!("Analyse project : {}", package_path);
    let reports_duplication =
        analyse_duplication(&package_path, &project_translations, &translations_indexed);
    print_global_duplication_report(&reports_duplication);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_settings_default_values() {
        let settings = Settings::default();
        assert!(!settings.translation_file_regex.is_empty());
        assert!(!settings.skip_directories.is_empty());
        assert!(settings.skip_directories.contains(&".git".to_string()));
    }

    #[test]
    fn test_path_handling() {
        let path = PathBuf::from("/test/path");
        assert!(path.to_string_lossy().contains("test"));
    }

    // Note: Full integration tests for global_report_all and global_report_for_project
    // would require setting up a test monorepo with translation files.
    // Consider adding these in tests/ directory with tempfile and fixture files.

    #[test]
    #[ignore] // Ignored by default as it requires file system setup
    fn test_global_report_integration() {
        // Example structure for integration test:
        // 1. Create temporary directory with TempDir
        // 2. Create sample translation files
        // 3. Create Settings with appropriate regex
        // 4. Call global_report_for_project
        // 5. Assert results (would need to capture stdout or refactor for testability)
    }
}
