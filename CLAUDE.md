# CLAUDE.md - Translation Analyzer

## Project Overview

**translations-analyzer** is a Rust CLI tool designed to analyze translation files in monorepo structures, identify duplications, and generate reports about translation usage across different packages and modules.

### Purpose
- Detect duplicate translations across packages
- Identify common translations that could be refactored
- Generate global and detailed reports for translation usage
- Optimize translation management in large codebases

## Repository Structure

```
translation-analyzer/
├── .github/
│   └── workflows/
│       ├── rust.yml           # CI workflow (build & test on main)
│       └── release.yml        # Release workflow for multi-platform binaries
├── src/
│   ├── main.rs                # CLI entry point and command handling
│   ├── entities.rs            # Core data structures (Translation, PackageType)
│   ├── settings.rs            # Configuration management
│   ├── load_translations.rs   # Parallel translation file loading
│   ├── search_recursive_regex.rs    # Recursive file search with regex
│   ├── map_translations_by_key.rs   # Translation indexing by content
│   ├── map_translations_by_project.rs # Project-based translation mapping
│   ├── analyse_project_duplication.rs # Duplication detection logic
│   └── get_translation_for_project.rs # Project-specific translation retrieval
├── Cargo.toml                 # Rust package manifest
├── Cargo.lock                 # Dependency lock file
├── rustfmt.toml              # Code formatting configuration
└── .gitignore                # Git ignore rules
```

## Core Architecture

### Module Organization

1. **main.rs** (src/main.rs:1-189)
   - Entry point with CLI argument parsing using `clap`
   - Defines commands: `GlobalReport`, `DetailedReport`
   - Command handlers: `global_report_all()`, `global_report_for_project()`, `detailled_report_for_project()`
   - Error handling with custom `CliError` enum

2. **entities.rs** (src/entities.rs:1-40)
   - `Translation` struct: Core data structure with path, translations (content), and key
   - `PackageType` enum: Apps or Modules classification
   - Serde serialization support for JSON handling

3. **settings.rs** (src/settings.rs:1-48)
   - `Settings` struct with configuration options:
     - `common_translations_modules_path`: Paths to shared translation modules
     - `translation_file_regex`: Regex pattern for translation files (default: `^Messages_fr_FR\.json$`)
     - `skip_directories`: Directories to exclude from search
   - Default configuration for common use cases

4. **load_translations.rs** (src/load_translations.rs:1-92)
   - Parallel translation loading using `rayon` for performance
   - JSON parsing with proper error handling
   - Thread-safe result collection with `parking_lot::Mutex`

5. **analyse_project_duplication.rs** (src/analyse_project_duplication.rs:1-50)
   - Duplication detection with three types:
     - `InterPackage`: Duplicates within the same project
     - `CommonTranslation`: Available in common-translations module
     - `ExternalProjects`: Used in other projects
   - Report generation and printing utilities

### Data Flow

```
1. CLI Command Parsing (clap)
   ↓
2. Load Settings (settings.json or defaults)
   ↓
3. Search Translation Files (recursive regex search)
   ↓
4. Load Translations (parallel JSON parsing)
   ↓
5. Index Translations (by content and by project)
   ↓
6. Analyze Duplications
   ↓
7. Generate Reports (global or detailed)
```

## Key Dependencies

From Cargo.toml:

- **clap** (v4.5.51): Command-line argument parsing with derive macros
- **serde** (v1.0.228) + **serde_json** (v1.0.139): JSON serialization/deserialization
- **rayon** (v1.8): Data parallelism for performance
- **parking_lot** (v0.12): More efficient synchronization primitives
- **regex** (v1.10): Pattern matching for file searches
- **thiserror** (v2.0.17): Ergonomic error handling
- **once_cell** (v1.21.3): Lazy static initialization
- **sysinfo** (v0.37.2): System information utilities

## Development Workflows

### Building the Project

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with arguments
cargo run -- global-report --package-path "packages/manager/apps/zimbra"
```

### Code Formatting

Formatting rules are defined in rustfmt.toml:
- `max_width = 90`: Maximum line width
- `chain_width = 70`: Maximum width for chained expressions
- `struct_lit_width = 50`: Maximum width for struct literals
- `use_field_init_shorthand = true`: Use shorthand for field initialization

Run formatting:
```bash
cargo fmt
```

### Running the Tool

```bash
# Global report for all projects
cargo run -- global-report

# Global report for specific project
cargo run -- global-report --package-path "packages/manager/apps/zimbra"

# Detailed report for specific project
cargo run -- detailed-report --package-path "packages/manager/apps/zimbra"

# With custom settings
cargo run -- --config-file-path custom-settings.json global-report

# With custom root path
cargo run -- --root-path /path/to/monorepo global-report
```

### Configuration File

Create a `settings.json` file in the project root:

```json
{
  "common_translations_modules_path": [
    "packages/manager/modules/common-translations"
  ],
  "translation_file_regex": "^Messages_fr_FR\\.json$",
  "skip_directories": [
    ".git",
    "node_modules",
    "target",
    ".idea",
    ".vscode",
    "dist",
    "build",
    "manager-tools"
  ]
}
```

## CI/CD Workflows

### Continuous Integration (.github/workflows/rust.yml)

Runs on push/PR to main branch:
1. Checkout code
2. Build with `cargo build --verbose`
3. Run tests with `cargo test --verbose`

### Release Workflow (.github/workflows/release.yml)

Triggered on GitHub release creation:
- Builds for multiple platforms:
  - x86_64-pc-windows-gnu (zip)
  - x86_64-unknown-linux-musl (tar.gz, tar.xz, tar.zst)
  - x86_64-apple-darwin (zip)
- Uses rust-build/rust-build.action@v1.4.5 for cross-compilation

## Code Conventions for AI Assistants

### Rust Best Practices

1. **Error Handling**
   - Use `thiserror` for custom error types
   - Always propagate errors with `?` operator or explicit handling
   - Avoid unwrap() in production code; use `expect()` with descriptive messages

2. **Naming Conventions**
   - Files: snake_case (e.g., `analyse_project_duplication.rs`)
   - Functions: snake_case (e.g., `load_translations()`)
   - Types: PascalCase (e.g., `Translation`, `DuplicationType`)
   - Constants: SCREAMING_SNAKE_CASE (e.g., `DEFAULT_SETTINGS_PATH_FILE`)

3. **Module Organization**
   - Each major functionality in separate module
   - Public API clearly marked with `pub` keyword
   - Use `pub(crate)` for internal API

4. **Performance Considerations**
   - Use `rayon` for parallelizable operations (file loading, searching)
   - Use `parking_lot::Mutex` instead of `std::sync::Mutex` for better performance
   - Clone only when necessary; prefer references with lifetimes

5. **Serialization**
   - Use `serde` derive macros for standard cases
   - Implement custom serializers for special cases (e.g., `PathBuf` with `serialize_path_lossy`)
   - Use `#[serde(rename_all = "lowercase")]` for consistent JSON formats

### Code Style

1. **Formatting**
   - Always run `cargo fmt` before committing
   - Follow the rules in rustfmt.toml
   - Keep lines under 90 characters

2. **Documentation**
   - Add doc comments (///) for public APIs
   - Include usage examples in doc comments where helpful
   - Document non-obvious implementation details with regular comments (//)

3. **Testing**
   - Write unit tests in the same file using `#[cfg(test)]` modules
   - Use descriptive test names that explain what is being tested
   - Test error cases, not just happy paths

### Common Patterns in This Codebase

1. **Parallel Processing Pattern**
   ```rust
   let results = Arc::new(parking_lot::Mutex::new(Vec::new()));
   items.par_iter().for_each(|item| {
       // Process item
       results.lock().push(result);
   });
   ```

2. **Error Propagation Pattern**
   ```rust
   fn operation() -> Result<Data, CustomError> {
       let data = risky_operation()
           .map_err(|e| CustomError::SpecificError("context", e))?;
       Ok(data)
   }
   ```

3. **CLI Subcommand Pattern**
   ```rust
   #[derive(Subcommand)]
   enum Commands {
       CommandName {
           #[arg(long)]
           param: Option<String>,
       },
   }

   match &cli.command {
       Some(Commands::CommandName { param }) => handle_command(param),
       // ...
   }
   ```

### When Making Changes

1. **Adding New Features**
   - Create a new module in `src/` for substantial features
   - Add module declaration in `main.rs`
   - Update CLI enum if adding new commands
   - Update settings if new configuration needed

2. **Modifying Existing Code**
   - Maintain backward compatibility with settings.json format
   - Update error messages to be descriptive
   - Consider performance impact (use parallel processing where beneficial)

3. **Adding Dependencies**
   - Prefer well-maintained crates with good documentation
   - Check compatibility with current Rust edition (2024)
   - Update Cargo.toml with specific version numbers

4. **Refactoring**
   - Keep module boundaries clean
   - Avoid circular dependencies between modules
   - Extract common functionality into shared utilities

### Git Workflow

1. **Branching**
   - Feature branches: `claude/feature-description-session-id`
   - Always work on the specified branch, never push to main directly

2. **Commits**
   - Use conventional commit messages: `feat:`, `fix:`, `refactor:`, `docs:`, `ci:`
   - Reference issue numbers when applicable
   - Keep commits atomic and focused

3. **Before Pushing**
   - Run `cargo build` to ensure compilation
   - Run `cargo test` to ensure tests pass
   - Run `cargo fmt` to format code
   - Run `cargo clippy` for linting suggestions

## Testing Strategy

Currently, the project has minimal test coverage. When adding tests:

1. **Unit Tests**
   - Add `#[cfg(test)]` modules at the end of each source file
   - Test individual functions in isolation
   - Mock file system operations where appropriate

2. **Integration Tests**
   - Create `tests/` directory for integration tests
   - Test complete workflows (search → load → analyze → report)
   - Use temporary directories for file-based tests

3. **Example Test Structure**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_function_name() {
           // Arrange
           let input = prepare_input();

           // Act
           let result = function_under_test(input);

           // Assert
           assert_eq!(result, expected_value);
       }
   }
   ```

## Common Operations Reference

### Adding a New CLI Command

1. Add variant to `Commands` enum in main.rs
2. Implement handler function
3. Add match arm in `main()` function
4. Update this documentation

### Adding a New Duplication Type

1. Add variant to `DuplicationType` enum in analyse_project_duplication.rs
2. Update `analyse_duplication()` logic
3. Update `print_global_duplication_report()` to include new type
4. Add tests for the new type

### Modifying Translation Search Logic

1. Update regex pattern in Settings or via CLI
2. Modify `search_recursive_regex()` if needed
3. Ensure skip_directories list is comprehensive
4. Test with various monorepo structures

## Troubleshooting

### Common Issues

1. **Settings not found**: Ensure `settings.json` exists or defaults will be used
2. **No files found**: Check regex pattern and skip_directories configuration
3. **Performance issues**: Verify rayon is being used for large file sets
4. **JSON parsing errors**: Ensure translation files are valid JSON objects

### Debugging

```bash
# Enable verbose logging (if implemented)
RUST_LOG=debug cargo run -- global-report

# Check what files are being searched
# Add debug prints in search_recursive_regex.rs

# Verify settings
cat settings.json
```

## Future Improvements

Potential areas for enhancement:

1. Add comprehensive unit and integration tests
2. Implement caching for large monorepos
3. Add support for multiple translation file formats (YAML, XML, etc.)
4. Create a web interface for visualization
5. Add configuration validation
6. Implement incremental analysis (only changed files)
7. Add translation coverage metrics
8. Support for multiple languages in single analysis
9. Export reports in different formats (JSON, CSV, HTML)
10. Add translation similarity detection (fuzzy matching)

## Additional Resources

- Rust documentation: https://doc.rust-lang.org/
- Clap documentation: https://docs.rs/clap/
- Rayon guide: https://docs.rs/rayon/
- Serde guide: https://serde.rs/

---

Last Updated: 2025-11-14
Rust Edition: 2024
