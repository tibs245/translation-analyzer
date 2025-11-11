mod search_recursive_regex;
mod load_translations;
mod map_translations_by_key;
mod entities;
mod map_translations_by_project;
mod analyse_project_duplication;

use sysinfo::{Pid, System};
use std::path::Path;
use std::time::Instant;
use crate::analyse_project_duplication::{analyse_duplication, print_global_duplication_report};
use crate::load_translations::load_translations;
use crate::map_translations_by_key::map_translations_by_translation;
use crate::map_translations_by_project::map_translations_by_project;
use crate::search_recursive_regex::search_recursive_regex;

// Root project : /Users/tibs/Workspace/ovh/manager
// Common translations : packages/manager/modules/common-translations
// Module : packages/manager/modules/*/
// Apps : packages/manager/apps/*/

fn print_memory_usage() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let current_pid = Pid::from(std::process::id() as usize);
    if let Some(process) = sys.process(current_pid) {
        println!("\nMemory used: {} MB", process.memory() / 1024 / 1024);
    }
}

fn main() {
    let start = Instant::now();

    let matches = search_recursive_regex(
        Path::new("/Users/tibs/Workspace/ovh/manager"),
        r#"^Messages_fr_FR\.json$"#,
    ).unwrap();

    println!("Files founds : {}", matches.len());

    let duration = start.elapsed();
    println!("File search Execution time: {:.2}ms", duration.as_secs_f64() * 1000.0);

    let start_loading = Instant::now();

    let translations = load_translations(matches).expect("Cannot map translations");

    let duration_loading = start_loading.elapsed();
    println!("Loading Execution time: {:.2}ms", duration_loading.as_secs_f64() * 1000.0);

    println!("Translations founds : {}", translations.len());

    print_memory_usage();

    let start_mapping = Instant::now();

    let translation_mapped = map_translations_by_translation(&translations);

    let duration_mapping = start_mapping.elapsed();
    println!("Mapping Execution time: {:.2}ms", duration_mapping.as_secs_f64() * 1000.0);


    print_memory_usage();

    let start_project_mapping = Instant::now();

    let mapped_by_project = map_translations_by_project(&translations);

    println!("Project founded : {}", mapped_by_project.len());

    let duration_project_mapping = start_project_mapping.elapsed();
    println!("Project Mapping Execution time: {:.2}ms", duration_project_mapping.as_secs_f64() * 1000.0);

    print_memory_usage();

    let start_analyse_project = Instant::now();

    for project_path in mapped_by_project.keys() {
        println!("Analyse project : {}", project_path);
        let reports_duplication = analyse_duplication(&project_path, &mapped_by_project[project_path], &translation_mapped);
        print_global_duplication_report(&reports_duplication);
    }

    let duration_analyse_project = start_analyse_project.elapsed();
    println!("Project Analyse Execution time: {:.2}ms", duration_analyse_project.as_secs_f64() * 1000.0);

    let duration_total = start.elapsed();
    println!("Total Execution time: {:.2}ms", duration_total.as_secs_f64() * 1000.0);

    print_memory_usage();


}
