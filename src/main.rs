// CLI binary - only compiled with 'cli' feature
#![cfg(feature = "cli")]

use translations_analyzer::{
    Settings, detailed_report_for_project, global_report_all, global_report_for_project,
};

use clap::{Parser, Subcommand};
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use thiserror::Error;

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

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Init invoices path
    GlobalReport {
        /// Sets a custom package path folder as `packages/manager/apps/zimbra` or `packages/manager/modules/backup-agent`
        #[arg(long)]
        package_path: Option<String>,
    },
    DetailedReport {
        /// Sets a custom package path folder as `packages/manager/apps/zimbra` or `packages/manager/modules/backup-agent`
        #[arg(long)]
        package_path: Option<String>,
    },
}

const DEFAULT_SETTINGS_PATH_FILE: &str = "settings.json";

fn main() {
    let cli = Cli::parse();

    let current_dir = env::current_dir().unwrap();
    let monorepo_path = cli.root_path.as_deref().unwrap_or(current_dir.as_path());

    println!("Root path : {}", monorepo_path.to_string_lossy());

    let config_file_path = cli
        .config_file_path
        .as_deref()
        .unwrap_or(Path::new(DEFAULT_SETTINGS_PATH_FILE));

    let config = settings::get_settings(config_file_path).unwrap_or(Settings::default());

    let result: Result<(), Box<dyn Error + Sync + Send + 'static>> = match &cli.command {
        Some(Commands::GlobalReport { package_path }) => match package_path {
            Some(package_path) => {
                global_report_for_project(monorepo_path, config, package_path)
            }
            None => global_report_all(monorepo_path, config),
        },
        Some(Commands::DetailedReport { package_path }) => match package_path {
            Some(package_path) => {
                detailed_report_for_project(monorepo_path, config, package_path)
            }
            None => Err(Box::new(CliError::NotImplementedYet())),
        },
        None => Err(Box::new(CliError::CommandNotExists(
            "The option is not correct. Try to get help".to_string(),
        ))),
    };

    result.unwrap_or_else(|error| println!("Error : {}", error));
}
