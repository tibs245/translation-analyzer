mod search_recursive_regex;
mod load_translations;
mod map_translations_by_key;
mod entities;
mod map_translations_by_project;
mod analyse_project_duplication;
mod settings;

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use thiserror::Error;
use crate::analyse_project_duplication::{analyse_duplication, print_global_duplication_report};
use crate::load_translations::load_translations;
use crate::map_translations_by_key::map_translations_by_translation;
use crate::map_translations_by_project::map_translations_by_project;
use crate::search_recursive_regex::search_recursive_regex;
use crate::settings::Settings;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Customer {0} not found")]
    CustomerNotFound(String),

    #[error("Not implemented yet")]
    NotImplementedYet(),

    #[error("{0}")]
    CommandNotExists(String),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom default root path
    #[arg(long, value_name = "FILE")]
    root_path: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(long, value_name = "FILE")]
    config_file_path: Option<PathBuf>,

    /// Sets a custom package path folder as `packages/manager/apps/zimbra` or `packages/manager/modules/backup-agent`
    #[arg(long, value_name = "FILE")]
    package_path: Option<PathBuf>,


    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Init invoices path
    GlobalReport,
}

const DEFAULT_SETTINGS_PATH_FILE: &str = "settings.json";

fn main() {
    let cli = Cli::parse();

    let current_dir = env::current_dir().unwrap();
    let monorepo_path = cli.root_path.as_deref().unwrap_or(current_dir.as_path());
    
    println!("Root path : {}", monorepo_path.to_string_lossy());

    let config_file_path = cli.config_file_path.as_deref().unwrap_or(Path::new(DEFAULT_SETTINGS_PATH_FILE));
    
    let config = settings::get_settings(config_file_path).unwrap_or(Settings::default());


    let result: Result<(), Box<dyn Error + Sync + Send + 'static>> = match &cli.command {
        Some(Commands::GlobalReport) => global_report_all(monorepo_path, config),
        None => Err(Box::new(CliError::CommandNotExists("The option is not correct. Try to get help".to_string())))
    };

    result.unwrap_or_else(|error| println!("Error : {}", error));
}



fn global_report_all(monorepo_path: &Path, config: Settings) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    let matches = search_recursive_regex(
        monorepo_path,
        &config.translation_file_regex,
        &config.skip_directories
    ).unwrap();
    println!("Found {} files", matches.len());

    let translations = load_translations(matches).expect("Cannot map translations");

    let translation_mapped = map_translations_by_translation(&translations);

    let mapped_by_project = map_translations_by_project(&translations);

    for project_path in mapped_by_project.keys() {
        println!("Analyse project : {}", project_path);
        let reports_duplication = analyse_duplication(&project_path, &mapped_by_project[project_path], &translation_mapped);
        print_global_duplication_report(&reports_duplication);
    }

    Ok(())
}
