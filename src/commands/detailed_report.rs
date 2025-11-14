use crate::analyse_project_duplication::{
    analyse_duplication, print_global_duplication_report,
};
use crate::get_translation_for_project::get_translations_for_project;
use crate::load_translations::load_translations;
use crate::map_translations_by_key::map_translations_by_translation;
use crate::map_translations_by_project::get_package_path;
use crate::search_recursive_regex::search_recursive_regex;
use crate::settings::Settings;
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;

/// Generate detailed report for a specific project
pub fn detailed_report_for_project(
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

    let mut displayed_translations: HashSet<String> = HashSet::new();

    for duplication in reports_duplication {
        if !displayed_translations.insert(duplication.translation.translations.clone()) {
            continue;
        }
        println!("\n");

        let other_usages = translations_indexed
            .get(&duplication.translation.translations)
            .unwrap();

        println!(
            " ========= Duplication seen : {} times, type : {:?} ==========",
            other_usages.len(),
            duplication.duplication_type
        );
        println!(
            " ========= {} ==========",
            duplication.translation.translations
        );

        for other_usage in other_usages {
            println!(
                "{} {} - {}",
                add_star_if_own_package(
                    package_path,
                    &other_usage.path.to_string_lossy().to_string()
                ),
                other_usage
                    .path
                    .strip_prefix(&monorepo_path)
                    .unwrap()
                    .to_string_lossy(),
                other_usage.key
            );
        }
    }

    println!("\n\n");

    Ok(())
}

/// Helper function to add a star marker if translation belongs to the package
fn add_star_if_own_package(package_path: &str, translations_path: &str) -> String {
    if get_package_path(translations_path) == package_path {
        return "**".to_string();
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_star_if_own_package_matches() {
        // Test when the translation path belongs to the same package
        let result = add_star_if_own_package(
            "packages/manager/apps/zimbra",
            "packages/manager/apps/zimbra/Messages_fr_FR.json",
        );
        assert_eq!(result, "**");
    }

    #[test]
    fn test_add_star_if_own_package_no_match() {
        // Test when the translation path belongs to a different package
        let result = add_star_if_own_package(
            "packages/manager/apps/zimbra",
            "packages/manager/apps/mail/Messages_fr_FR.json",
        );
        assert_eq!(result, "");
    }

    #[test]
    fn test_add_star_if_own_package_modules_vs_apps() {
        // Test different package types (modules vs apps)
        let result = add_star_if_own_package(
            "packages/manager/apps/zimbra",
            "packages/manager/modules/common-translations/Messages_fr_FR.json",
        );
        assert_eq!(result, "");
    }

    #[test]
    fn test_add_star_if_own_package_same_module() {
        // Test matching module paths
        let result = add_star_if_own_package(
            "packages/manager/modules/backup-agent",
            "packages/manager/modules/backup-agent/translations/Messages_fr_FR.json",
        );
        assert_eq!(result, "**");
    }

    #[test]
    fn test_add_star_if_own_package_nested_paths() {
        // Test with nested directory structures
        let result = add_star_if_own_package(
            "packages/manager/apps/zimbra",
            "packages/manager/apps/zimbra/src/components/Messages_fr_FR.json",
        );
        assert_eq!(result, "**");
    }

    #[test]
    fn test_add_star_if_own_package_unknown_path() {
        // Test with paths that don't match the expected pattern
        let result = add_star_if_own_package(
            "packages/manager/apps/zimbra",
            "some/random/path/Messages_fr_FR.json",
        );
        assert_eq!(result, "");
    }

    #[test]
    fn test_hashset_deduplication() {
        // Test HashSet behavior used in detailed_report_for_project
        let mut displayed: HashSet<String> = HashSet::new();

        // First insertion returns true
        assert!(displayed.insert("translation1".to_string()));

        // Duplicate insertion returns false
        assert!(!displayed.insert("translation1".to_string()));

        // Different value returns true
        assert!(displayed.insert("translation2".to_string()));
    }

    #[test]
    fn test_detailed_report_for_project_integration() {
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

        // Create translation files with duplicates for testing
        zimbra_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "welcome.title": "Bienvenue dans Zimbra",
                "shared.save": "Enregistrer",
                "duplicate.across": "Texte partagé"
            }"#,
            )
            .unwrap();

        mail_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "mail.subject": "Sujet du mail",
                "duplicate.across": "Texte partagé",
                "shared.save": "Enregistrer"
            }"#,
            )
            .unwrap();

        common_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "common.close": "Fermer",
                "shared.save": "Enregistrer"
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

        // Run the detailed report - should not panic
        let result = detailed_report_for_project(
            temp_dir.path(),
            settings,
            "packages/manager/apps/zimbra",
        );

        // Assert it runs successfully
        assert!(result.is_ok(), "detailed_report_for_project should succeed");

        // Cleanup is automatic with TempDir
    }

    #[test]
    fn test_detailed_report_with_inter_package_duplicates() {
        use assert_fs::TempDir;
        use assert_fs::prelude::*;

        // Test specifically for inter-package duplication detection
        let temp_dir = TempDir::new().unwrap();

        let zimbra_dir = temp_dir.child("packages/manager/apps/zimbra");
        zimbra_dir.create_dir_all().unwrap();

        // Create two translation files in the same project
        zimbra_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "duplicate.internal": "Duplication interne"
            }"#,
            )
            .unwrap();

        zimbra_dir.child("subfolder").create_dir_all().unwrap();

        zimbra_dir
            .child("subfolder/Messages_fr_FR.json")
            .write_str(
                r#"{
                "duplicate.internal": "Duplication interne"
            }"#,
            )
            .unwrap();

        let settings = Settings {
            common_translations_modules_path: vec![],
            translation_file_regex: r"Messages_fr_FR\.json$".to_string(),
            skip_directories: vec![],
        };

        let result = detailed_report_for_project(
            temp_dir.path(),
            settings,
            "packages/manager/apps/zimbra",
        );

        assert!(result.is_ok(), "Should detect inter-package duplicates");
    }

    #[test]
    fn test_detailed_report_with_no_duplicates() {
        use assert_fs::TempDir;
        use assert_fs::prelude::*;

        // Test with unique translations only
        let temp_dir = TempDir::new().unwrap();

        let zimbra_dir = temp_dir.child("packages/manager/apps/zimbra");
        zimbra_dir.create_dir_all().unwrap();

        let mail_dir = temp_dir.child("packages/manager/apps/mail");
        mail_dir.create_dir_all().unwrap();

        zimbra_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "unique.zimbra": "Texte unique Zimbra"
            }"#,
            )
            .unwrap();

        mail_dir
            .child("Messages_fr_FR.json")
            .write_str(
                r#"{
                "unique.mail": "Texte unique Mail"
            }"#,
            )
            .unwrap();

        let settings = Settings {
            common_translations_modules_path: vec![],
            translation_file_regex: r"Messages_fr_FR\.json$".to_string(),
            skip_directories: vec![],
        };

        let result = detailed_report_for_project(
            temp_dir.path(),
            settings,
            "packages/manager/apps/zimbra",
        );

        // Should succeed even with no duplicates
        assert!(result.is_ok(), "Should handle no duplicates gracefully");
    }
}
