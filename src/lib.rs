mod analyse_project_duplication;
mod commands;
mod entities;
mod get_translation_for_project;
mod load_translations;
mod map_translations_by_key;
mod map_translations_by_project;
mod search_recursive_regex;
mod settings;

pub use settings::Settings;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

// Re-export command functions for native use
pub use commands::detailed_report::detailed_report_for_project;
pub use commands::global_report::{global_report_all, global_report_for_project};
