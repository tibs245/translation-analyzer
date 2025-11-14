# Commands Module Documentation

## Overview

The `commands` module contains the implementation of all CLI commands for the translation-analyzer tool. This modular structure separates command logic from CLI parsing, making the codebase more maintainable and testable.

## Architecture

```
src/commands/
├── mod.rs                  # Module exports
├── global_report.rs        # Global report commands
├── detailed_report.rs      # Detailed report commands
└── CLAUDE.md              # This file
```

### Design Principles

1. **Separation of Concerns**: Each command category has its own module
2. **Single Responsibility**: Each function handles one specific command variant
3. **Consistent Error Handling**: All commands return `Result<(), Box<dyn Error + Sync + Send>>`
4. **Minimal Dependencies**: Import only what's needed for each command
5. **Documentation**: All public functions have doc comments

## Module Structure

### mod.rs

The module entry point that exports all command modules:

```rust
pub mod global_report;
pub mod detailed_report;
```

**Best Practices:**
- Keep exports explicit and organized
- Add new command modules here as they're created
- Consider grouping related commands under submodules if the number grows

## Commands Overview

### 1. Global Report Commands (global_report.rs)

**Purpose**: Generate high-level duplication reports across projects

**Functions:**

#### `global_report_all()`
```rust
pub fn global_report_all(
    monorepo_path: &Path,
    config: Settings,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>>
```

**What it does:**
1. Searches for all translation files in the monorepo
2. Loads and indexes all translations
3. Maps translations by project
4. Analyzes duplication for each project
5. Prints summary reports for all projects

**Use case**: Quick overview of duplication across entire monorepo

**Example output:**
```
Found 150 files
Analyse project : packages/manager/apps/zimbra
Global duplication report :
Inter-package duplication : 5
Common-translation duplication : 23
External-projects duplication : 12
Total duplication : 40
```

#### `global_report_for_project()`
```rust
pub fn global_report_for_project(
    monorepo_path: &Path,
    config: Settings,
    package_path: &str,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>>
```

**What it does:**
1. Searches for all translation files in the monorepo
2. Loads and indexes all translations
3. Filters translations for the specified project
4. Analyzes duplication for that project only
5. Prints summary report

**Use case**: Focus on a specific project's duplication issues

**Parameters:**
- `monorepo_path`: Root path of the monorepo
- `config`: Settings with regex patterns and skip directories
- `package_path`: Specific project path (e.g., "packages/manager/apps/zimbra")

### 2. Detailed Report Commands (detailed_report.rs)

**Purpose**: Provide in-depth analysis with specific duplication details

**Functions:**

#### `detailed_report_for_project()`
```rust
pub fn detailed_report_for_project(
    monorepo_path: &Path,
    config: Settings,
    package_path: &str,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>>
```

**What it does:**
1. Performs same initial analysis as global report
2. Prints global summary
3. Lists each duplicated translation with:
   - Number of occurrences
   - Duplication type
   - All file locations where it appears
   - Translation keys
4. Marks files belonging to the analyzed project with "**"

**Use case**: Deep dive into duplication for refactoring decisions

**Example output:**
```
 ========= Duplication seen : 3 times, type : InterPackage ==========
 ========= "Welcome to the application" ==========
** packages/manager/apps/zimbra/Messages_fr_FR.json - welcome.title
   packages/manager/apps/mail/Messages_fr_FR.json - app.welcome
   packages/manager/modules/common/Messages_fr_FR.json - common.welcome
```

#### `add_star_if_own_package()` (Helper)
```rust
fn add_star_if_own_package(package_path: &str, translations_path: &str) -> String
```

**Purpose**: Visual indicator for translations belonging to the analyzed project

**Returns**: "**" if path matches package_path, empty string otherwise

## Common Patterns

### 1. Standard Command Flow

All commands follow this pattern:

```rust
pub fn command_name(
    monorepo_path: &Path,
    config: Settings,
    // ... additional params
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    // 1. Search for translation files
    let matches = search_recursive_regex(
        monorepo_path,
        &config.translation_file_regex,
        &config.skip_directories,
    )?;

    // 2. Load translations (parallel processing)
    let translations = load_translations(matches)?;

    // 3. Index translations for fast lookup
    let translations_indexed = map_translations_by_translation(&translations);

    // 4. Filter/organize as needed
    let project_translations = get_translations_for_project(package_path, &translations);

    // 5. Analyze duplications
    let reports = analyse_duplication(&package_path, &project_translations, &translations_indexed);

    // 6. Display results
    print_global_duplication_report(&reports);

    Ok(())
}
```

### 2. Error Handling

**Current approach**: Use `.unwrap()` and `.expect()` for operations that should not fail in normal operation

**Better approach for production**:
```rust
let matches = search_recursive_regex(
    monorepo_path,
    &config.translation_file_regex,
    &config.skip_directories,
)
.map_err(|e| format!("Failed to search files: {}", e))?;

let translations = load_translations(matches)
    .map_err(|e| format!("Failed to load translations: {}", e))?;
```

### 3. Import Organization

Commands should import only what they need:

```rust
// Core functionality imports
use crate::analyse_project_duplication::{analyse_duplication, print_global_duplication_report};
use crate::load_translations::load_translations;
use crate::search_recursive_regex::search_recursive_regex;
use crate::settings::Settings;

// Standard library imports
use std::collections::HashSet;  // Only if needed
use std::error::Error;
use std::path::Path;
```

## Adding New Commands

### Step 1: Create Command Module

Create a new file `src/commands/new_command.rs`:

```rust
use crate::settings::Settings;
use std::error::Error;
use std::path::Path;

/// Brief description of what this command does
pub fn new_command(
    monorepo_path: &Path,
    config: Settings,
    // Add parameters as needed
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    // Implementation
    Ok(())
}
```

### Step 2: Export in mod.rs

Add to `src/commands/mod.rs`:

```rust
pub mod new_command;
```

### Step 3: Wire Up in main.rs

1. Add command variant to `Commands` enum:
```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands
    NewCommand {
        #[arg(long)]
        param: Option<String>,
    },
}
```

2. Import and use in main():
```rust
use crate::commands::new_command::new_command;

// In match statement:
Some(Commands::NewCommand { param }) => {
    new_command(monorepo_path, config, param.as_deref().unwrap_or("default"))
}
```

### Step 4: Add Tests

Add unit tests at the bottom of your command file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_new_command() {
        // Test implementation
    }
}
```

## Testing Commands

### Unit Testing Strategy

Commands are integration points, so testing can be challenging. Here are approaches:

#### 1. Test Helper Functions

Extract testable logic into helper functions:

```rust
// Easy to test
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
        let result = add_star_if_own_package(
            "packages/manager/apps/zimbra",
            "packages/manager/apps/zimbra/Messages_fr_FR.json"
        );
        assert_eq!(result, "**");
    }

    #[test]
    fn test_add_star_if_own_package_no_match() {
        let result = add_star_if_own_package(
            "packages/manager/apps/zimbra",
            "packages/manager/apps/mail/Messages_fr_FR.json"
        );
        assert_eq!(result, "");
    }
}
```

#### 2. Integration Tests

For full command testing, use integration tests in `tests/` directory:

```rust
// tests/command_integration_test.rs
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use translations_analyzer::commands::global_report::global_report_for_project;
use translations_analyzer::settings::Settings;

#[test]
fn test_global_report_with_real_files() {
    let temp_dir = TempDir::new().unwrap();
    // Set up test files
    // Run command
    // Assert results
}
```

#### 3. Mock Testing

For commands with side effects (file I/O, printing), consider:
- Using traits for testability
- Dependency injection
- Capturing stdout for assertions

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific module
cargo test commands::global_report

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_add_star_if_own_package
```

## Best Practices for Command Development

### 1. Function Naming

- Use verb-noun pattern: `generate_report()`, `analyze_project()`, `export_data()`
- Be specific: `global_report_for_project()` not just `report()`
- Match CLI command names when possible

### 2. Parameter Order

Standard order for consistency:
```rust
fn command(
    monorepo_path: &Path,      // 1. Input paths (immutable references)
    config: Settings,           // 2. Configuration (owned or reference)
    specific_params: &str,      // 3. Command-specific parameters
) -> Result<(), Box<dyn Error + Sync + Send + 'static>>
```

### 3. Documentation

Every public function should have:
```rust
/// Brief one-line description
///
/// Longer explanation of what the function does,
/// how it processes data, and any important details.
///
/// # Arguments
///
/// * `monorepo_path` - Root path of the monorepo
/// * `config` - Settings including regex patterns and skip directories
/// * `package_path` - Specific project path to analyze
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if file operations fail
///
/// # Examples
///
/// ```no_run
/// let result = global_report_for_project(
///     Path::new("/path/to/monorepo"),
///     Settings::default(),
///     "packages/manager/apps/zimbra"
/// );
/// ```
pub fn command_name() -> Result<(), Box<dyn Error>> {
    // Implementation
}
```

### 4. Error Messages

Provide context in error messages:
```rust
// Bad
.expect("Failed");

// Good
.expect("Failed to load translations from matched files");

// Better (propagate with context)
.map_err(|e| format!("Failed to load translations from {}: {}", path.display(), e))?;
```

### 5. Output Formatting

- Use consistent prefixes for different output types
- Make output parseable if it might be consumed by other tools
- Consider adding quiet/verbose modes via config
- Use colors sparingly (consider NO_COLOR environment variable)

```rust
// Current style - simple and clear
println!("Analyse project : {}", package_path);
println!("Global duplication report :");
println!("Inter-package duplication : {}", count);

// Enhanced style (optional)
println!("[INFO] Analyzing project: {}", package_path);
println!("[REPORT] Global duplication report:");
println!("  Inter-package:      {}", count);
println!("  Common-translation: {}", count);
```

### 6. Performance Considerations

- Leverage parallel processing (already done in `load_translations`)
- Avoid unnecessary cloning of large data structures
- Use references where possible
- Consider streaming for very large datasets

```rust
// Good - reuse indexed translations
let translations_indexed = map_translations_by_translation(&translations);
for package in packages {
    analyse_duplication(&package, &translations_indexed);  // Reference
}

// Bad - would reindex for each package
for package in packages {
    let indexed = map_translations_by_translation(&translations);  // Wasteful
}
```

## Future Enhancements

### Command Ideas

1. **Export Command**: Export results to JSON/CSV
2. **Compare Command**: Compare duplication between two time points
3. **Suggest Command**: AI-powered suggestions for refactoring
4. **Stats Command**: Statistical analysis of translations
5. **Validate Command**: Check for missing translations

### Architectural Improvements

1. **Trait-based Commands**: Define a `Command` trait for consistency
2. **Builder Pattern**: For commands with many parameters
3. **Progress Indicators**: For long-running operations
4. **Streaming Output**: For real-time feedback
5. **Caching**: Cache translation loading for repeated analyses

### Example Command Trait

```rust
pub trait Command {
    type Args;
    type Output;

    fn execute(&self, args: Self::Args) -> Result<Self::Output, Box<dyn Error>>;
    fn validate_args(&self, args: &Self::Args) -> Result<(), String>;
}

impl Command for GlobalReportCommand {
    type Args = GlobalReportArgs;
    type Output = ();

    fn execute(&self, args: Self::Args) -> Result<(), Box<dyn Error>> {
        // Implementation
    }
}
```

## Troubleshooting

### Common Issues

1. **Command panics with unwrap()**
   - Check that translation files exist and are valid JSON
   - Verify package_path matches actual directory structure

2. **No duplications found**
   - Verify translation_file_regex matches your file names
   - Check skip_directories isn't excluding translation files
   - Ensure translations actually have duplicate content

3. **Memory issues with large monorepos**
   - Consider implementing streaming for file processing
   - Add limits on batch sizes
   - Profile with `cargo flamegraph`

## Contributing

When adding or modifying commands:

1. Follow the existing patterns and structure
2. Add comprehensive documentation
3. Write unit tests for helper functions
4. Consider integration tests for full command flows
5. Update this CLAUDE.md with new patterns or commands
6. Run `cargo fmt` and `cargo clippy` before committing
7. Update the root CLAUDE.md if adding new architectural patterns

## References

- Main CLAUDE.md: `../CLAUDE.md` - Overall project documentation
- Settings module: `../settings.rs` - Configuration structure
- Analysis module: `../analyse_project_duplication.rs` - Duplication detection logic
- Clap documentation: https://docs.rs/clap/ - CLI parsing framework

---

Last Updated: 2025-11-14
