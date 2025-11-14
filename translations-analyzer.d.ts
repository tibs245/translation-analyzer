/**
 * TypeScript type definitions for translations-analyzer WASM module
 *
 * This module provides functionality to analyze translation files in monorepo structures,
 * identify duplications, and generate reports.
 */

/**
 * Options for configuring the translation analyzer
 */
export class AnalyzerOptions {
  /**
   * Create a new AnalyzerOptions instance with default values
   */
  constructor();

  /**
   * Regular expression pattern to match translation files
   * Default: "^Messages_fr_FR\\.json$"
   */
  translation_file_regex: string;

  /**
   * Array of directory names to skip during search
   * Default: [".git", "node_modules", "target", ".idea", ".vscode", "dist", "build", "manager-tools"]
   */
  skip_directories: string[];

  /**
   * Paths to common translation modules
   * Default: ["packages/manager/modules/common-translations"]
   */
  common_translations_modules_path: string[];
}

/**
 * Types of duplication detected in translations
 */
export type DuplicationType =
  | "InterPackage"       // Duplicates within the same project
  | "CommonTranslation"  // Available in common-translations module
  | "ExternalProjects";  // Used in other projects

/**
 * Information about a single duplicated translation
 */
export interface DuplicationReportData {
  /** The translation key */
  translation_key: string;

  /** The translation value/text */
  translation_value: string;

  /** File path where this translation was found */
  file_path: string;

  /** Type of duplication detected */
  duplication_type: DuplicationType;

  /** Number of times this translation appears */
  occurrences_count: number;
}

/**
 * Summary statistics for global duplication report
 */
export interface GlobalReportResult {
  /** Number of translation files found */
  files_found: number;

  /** Count of inter-package duplications */
  inter_package_duplication: number;

  /** Count of common translation duplications */
  common_translation_duplication: number;

  /** Count of external project duplications */
  external_projects_duplication: number;

  /** Total count of all duplications */
  total_duplication: number;
}

/**
 * Detailed duplication report with individual duplication entries
 */
export interface DetailedReportResult {
  /** Number of translation files found */
  files_found: number;

  /** Global statistics summary */
  global_report: GlobalReportResult;

  /** Array of individual duplication entries */
  duplications: DuplicationReportData[];
}

/**
 * Project-specific report in the global report for all projects
 */
export interface ProjectReport extends GlobalReportResult {
  /** Path to the package/project */
  package_path: string;
}

/**
 * Global report result for all projects
 */
export interface GlobalReportAllResult {
  /** Number of translation files found */
  files_found: number;

  /** Array of per-project reports */
  projects: ProjectReport[];
}

/**
 * Get a global duplication report for a specific project
 *
 * @param monorepoPath - Root path of the monorepo
 * @param packagePath - Specific project path (e.g., "packages/manager/apps/zimbra")
 * @param options - Configuration options for the analyzer
 * @returns Promise resolving to global report statistics
 *
 * @example
 * ```typescript
 * import init, { get_global_report_for_project, AnalyzerOptions } from './translations_analyzer';
 *
 * await init();
 *
 * const options = new AnalyzerOptions();
 * options.translation_file_regex = "Messages_fr_FR\\.json$";
 *
 * const report = await get_global_report_for_project(
 *   "/path/to/monorepo",
 *   "packages/manager/apps/zimbra",
 *   options
 * );
 *
 * console.log(`Total duplications: ${report.total_duplication}`);
 * ```
 */
export function get_global_report_for_project(
  monorepoPath: string,
  packagePath: string,
  options: AnalyzerOptions
): Promise<GlobalReportResult>;

/**
 * Get a detailed duplication report for a specific project
 *
 * Provides comprehensive information about each duplication including
 * file locations, translation keys, and duplication types.
 *
 * @param monorepoPath - Root path of the monorepo
 * @param packagePath - Specific project path (e.g., "packages/manager/apps/zimbra")
 * @param options - Configuration options for the analyzer
 * @returns Promise resolving to detailed report with duplication entries
 *
 * @example
 * ```typescript
 * import init, { get_detailed_report_for_project, AnalyzerOptions } from './translations_analyzer';
 *
 * await init();
 *
 * const options = new AnalyzerOptions();
 * const report = await get_detailed_report_for_project(
 *   "/path/to/monorepo",
 *   "packages/manager/apps/zimbra",
 *   options
 * );
 *
 * report.duplications.forEach(dup => {
 *   console.log(`${dup.translation_key}: ${dup.duplication_type} (${dup.occurrences_count} times)`);
 * });
 * ```
 */
export function get_detailed_report_for_project(
  monorepoPath: string,
  packagePath: string,
  options: AnalyzerOptions
): Promise<DetailedReportResult>;

/**
 * Get a global report for all projects in the monorepo
 *
 * Analyzes all projects found in the monorepo and provides statistics
 * for each project separately.
 *
 * @param monorepoPath - Root path of the monorepo
 * @param options - Configuration options for the analyzer
 * @returns Promise resolving to reports for all projects
 *
 * @example
 * ```typescript
 * import init, { get_global_report_all, AnalyzerOptions } from './translations_analyzer';
 *
 * await init();
 *
 * const options = new AnalyzerOptions();
 * const report = await get_global_report_all("/path/to/monorepo", options);
 *
 * report.projects.forEach(project => {
 *   console.log(`${project.package_path}: ${project.total_duplication} duplications`);
 * });
 * ```
 */
export function get_global_report_all(
  monorepoPath: string,
  options: AnalyzerOptions
): Promise<GlobalReportAllResult>;

/**
 * Initialize the WASM module
 * Must be called before using any other functions
 *
 * @example
 * ```typescript
 * import init from './translations_analyzer';
 *
 * await init();
 * // Now you can use other functions
 * ```
 */
export default function init(input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module): Promise<void>;
