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
