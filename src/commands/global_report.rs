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

    #[test]
    fn test_global_report_for_project_integration() {
        use assert_fs::TempDir;
        use assert_fs::prelude::*;

        // Create temporary directory structure
        let temp_dir = TempDir::new().unwrap();

        // Create packages structure
        let zimbra_dir = temp_dir.child("packages/manager/apps/zimbra");
        zimbra_dir.create_dir_all().unwrap();

        let mail_dir = temp_dir.child("packages/manager/apps/mail");
        mail_dir.create_dir_all().unwrap();

        let common_dir = temp_dir.child("packages/manager/modules/common-translations");
        common_dir.create_dir_all().unwrap();

        // Create translation files with sample content
        zimbra_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "welcome.title": "Bienvenue",
                "error.message": "Une erreur s'est produite",
                "duplicate.text": "Texte dupliqué"
            }"#,
            )
            .unwrap();

        mail_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "mail.title": "Courrier",
                "duplicate.text": "Texte dupliqué"
            }"#,
            )
            .unwrap();

        common_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "common.button": "Bouton commun"
            }"#,
            )
            .unwrap();

        // Create settings
        let settings = Settings {
            common_translations_modules_path: vec![
                "packages/manager/modules/common-translations".to_string(),
            ],
            translation_file_regex: r"Messages_fr_FR\.json$".to_string(),
            skip_directories: vec![".git".to_string(), "node_modules".to_string()],
        };

        // Run the command - should not panic
        let result = global_report_for_project(
            temp_dir.path(),
            settings,
            "packages/manager/apps/zimbra",
        );

        // Assert it runs successfully
        assert!(result.is_ok(), "global_report_for_project should succeed");

        // Cleanup is automatic with TempDir
    }

    #[test]
    fn test_global_report_all_integration() {
        use assert_fs::TempDir;
        use assert_fs::prelude::*;

        // Create temporary directory structure
        let temp_dir = TempDir::new().unwrap();

        // Create packages structure
        let zimbra_dir = temp_dir.child("packages/manager/apps/zimbra");
        zimbra_dir.create_dir_all().unwrap();

        let mail_dir = temp_dir.child("packages/manager/apps/mail");
        mail_dir.create_dir_all().unwrap();

        // Create translation files
        zimbra_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "app.title": "Application Zimbra"
            }"#,
            )
            .unwrap();

        mail_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "app.title": "Application Mail"
            }"#,
            )
            .unwrap();

        // Create settings
        let settings = Settings {
            common_translations_modules_path: vec![],
            translation_file_regex: r"Messages_fr_FR\.json$".to_string(),
            skip_directories: vec![".git".to_string()],
        };

        // Run the command - should not panic
        let result = global_report_all(temp_dir.path(), settings);

        // Assert it runs successfully
        assert!(result.is_ok(), "global_report_all should succeed");
    }

    #[test]
    fn test_global_report_with_no_files() {
        use assert_fs::TempDir;

        // Create empty temporary directory
        let temp_dir = TempDir::new().unwrap();

        let settings = Settings {
            common_translations_modules_path: vec![],
            translation_file_regex: r"Messages_fr_FR\.json$".to_string(),
            skip_directories: vec![],
        };

        // Run with no matching files - should handle gracefully
        let result = global_report_all(temp_dir.path(), settings);

        // Should succeed even with no files
        assert!(result.is_ok(), "Should handle empty directory gracefully");
    }
}
