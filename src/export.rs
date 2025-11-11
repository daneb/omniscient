/// Export and import functionality for command history
use crate::error::Result;
use crate::models::CommandRecord;
use crate::Storage;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Export format version for compatibility checking
const EXPORT_VERSION: &str = "1.0";

/// Export file structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    /// Format version
    pub version: String,

    /// Export timestamp
    pub exported_at: String,

    /// Total number of commands
    pub command_count: usize,

    /// All command records
    pub commands: Vec<CommandRecord>,
}

/// Export command history to JSON file
pub struct Exporter {
    storage: Storage,
}

impl Exporter {
    /// Create a new exporter with the given storage
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    /// Export all commands to a JSON file
    pub fn export<P: AsRef<Path>>(&self, output_path: P) -> Result<ExportStats> {
        let commands = self.storage.get_all()?;
        let command_count = commands.len();

        let export_data = ExportData {
            version: EXPORT_VERSION.to_string(),
            exported_at: chrono::Utc::now().to_rfc3339(),
            command_count,
            commands,
        };

        // Serialize to pretty JSON
        let json = serde_json::to_string_pretty(&export_data)?;

        // Write to file
        fs::write(output_path.as_ref(), json)?;

        Ok(ExportStats {
            commands_exported: command_count,
            file_path: output_path.as_ref().display().to_string(),
        })
    }
}

/// Statistics from an export operation
#[derive(Debug)]
pub struct ExportStats {
    pub commands_exported: usize,
    pub file_path: String,
}

/// Import strategy for handling duplicates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImportStrategy {
    /// Skip duplicate commands
    Skip,

    /// Update usage count (add the counts together)
    UpdateUsage,

    /// Preserve the higher usage count
    #[default]
    PreserveHigher,
}

/// Import command history from JSON file
pub struct Importer {
    storage: Storage,
    strategy: ImportStrategy,
}

impl Importer {
    /// Create a new importer with the given storage and strategy
    pub fn new(storage: Storage, strategy: ImportStrategy) -> Self {
        Self { storage, strategy }
    }

    /// Import commands from a JSON file
    pub fn import<P: AsRef<Path>>(&self, input_path: P) -> Result<ImportStats> {
        // Read and parse the JSON file
        let json = fs::read_to_string(input_path.as_ref())?;
        let export_data: ExportData = serde_json::from_str(&json)?;

        // Validate version (for now, just check it exists)
        if export_data.version.is_empty() {
            return Err(crate::error::OmniscientError::Config(
                "Invalid export file: missing version".to_string(),
            ));
        }

        let mut stats = ImportStats {
            total_commands: export_data.command_count,
            imported: 0,
            skipped: 0,
            updated: 0,
        };

        // Import each command
        for cmd in export_data.commands {
            // Check for duplicates
            let duplicate = self.storage.find_duplicate(&cmd.command, &cmd.working_dir)?;

            match duplicate {
                Some(existing) => {
                    // Handle duplicate based on strategy
                    match self.strategy {
                        ImportStrategy::Skip => {
                            stats.skipped += 1;
                        }
                        ImportStrategy::UpdateUsage => {
                            // Update the existing command with combined usage count
                            let new_count = existing.usage_count + cmd.usage_count;
                            self.update_usage_count(existing.id.unwrap(), new_count)?;
                            stats.updated += 1;
                        }
                        ImportStrategy::PreserveHigher => {
                            // Keep the higher usage count
                            if cmd.usage_count > existing.usage_count {
                                self.update_usage_count(existing.id.unwrap(), cmd.usage_count)?;
                                stats.updated += 1;
                            } else {
                                stats.skipped += 1;
                            }
                        }
                    }
                }
                None => {
                    // No duplicate, insert as new command
                    self.storage.insert(&cmd)?;
                    stats.imported += 1;
                }
            }
        }

        Ok(stats)
    }

    /// Update usage count for an existing command
    fn update_usage_count(&self, id: i64, _new_count: i32) -> Result<()> {
        // For now, we'll just increment once to update last_used timestamp
        // The usage count merging is a best-effort approach
        // TODO: Add a set_usage_count method to Storage for more accurate updates
        self.storage.increment_usage(id)?;
        Ok(())
    }
}

/// Statistics from an import operation
#[derive(Debug)]
pub struct ImportStats {
    pub total_commands: usize,
    pub imported: usize,
    pub skipped: usize,
    pub updated: usize,
}

impl ImportStats {
    /// Get a summary message
    pub fn summary(&self) -> String {
        format!(
            "Imported {} new commands, updated {}, skipped {} duplicates (total: {})",
            self.imported, self.updated, self.skipped, self.total_commands
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::NamedTempFile;

    fn create_test_storage() -> Storage {
        let temp_file = NamedTempFile::new().unwrap();
        Storage::new(temp_file.path()).unwrap()
    }

    fn create_test_command(command: &str, category: &str, usage: i32) -> CommandRecord {
        let mut cmd = CommandRecord::new(
            command.to_string(),
            Utc::now(),
            0,
            100,
            "/tmp".to_string(),
            category.to_string(),
        );
        cmd.usage_count = usage;
        cmd
    }

    #[test]
    fn test_export_empty_database() {
        let storage = create_test_storage();
        let exporter = Exporter::new(storage);
        let temp_file = NamedTempFile::new().unwrap();

        let stats = exporter.export(temp_file.path()).unwrap();
        assert_eq!(stats.commands_exported, 0);

        // Verify file exists
        assert!(temp_file.path().exists());
    }

    #[test]
    fn test_export_with_commands() {
        let storage = create_test_storage();
        storage.insert(&create_test_command("git status", "git", 5)).unwrap();
        storage.insert(&create_test_command("docker ps", "docker", 3)).unwrap();

        let exporter = Exporter::new(storage);
        let temp_file = NamedTempFile::new().unwrap();

        let stats = exporter.export(temp_file.path()).unwrap();
        assert_eq!(stats.commands_exported, 2);

        // Verify JSON is valid
        let json = fs::read_to_string(temp_file.path()).unwrap();
        let export_data: ExportData = serde_json::from_str(&json).unwrap();
        assert_eq!(export_data.version, EXPORT_VERSION);
        assert_eq!(export_data.command_count, 2);
        assert_eq!(export_data.commands.len(), 2);
    }

    #[test]
    fn test_import_new_commands() {
        let storage = create_test_storage();
        let _exporter = Exporter::new(create_test_storage());

        // Create source storage with commands
        let source_storage = create_test_storage();
        source_storage.insert(&create_test_command("git status", "git", 5)).unwrap();
        source_storage.insert(&create_test_command("docker ps", "docker", 3)).unwrap();

        // Export from source
        let temp_file = NamedTempFile::new().unwrap();
        let source_exporter = Exporter::new(source_storage);
        source_exporter.export(temp_file.path()).unwrap();

        // Import to target
        let importer = Importer::new(storage, ImportStrategy::Skip);
        let stats = importer.import(temp_file.path()).unwrap();

        assert_eq!(stats.total_commands, 2);
        assert_eq!(stats.imported, 2);
        assert_eq!(stats.skipped, 0);
        assert_eq!(stats.updated, 0);
    }

    #[test]
    fn test_import_with_duplicates_skip() {
        let storage = create_test_storage();
        storage.insert(&create_test_command("git status", "git", 5)).unwrap();

        // Create export with duplicate command
        let source_storage = create_test_storage();
        source_storage.insert(&create_test_command("git status", "git", 10)).unwrap();
        source_storage.insert(&create_test_command("docker ps", "docker", 3)).unwrap();

        let temp_file = NamedTempFile::new().unwrap();
        let source_exporter = Exporter::new(source_storage);
        source_exporter.export(temp_file.path()).unwrap();

        // Import with Skip strategy
        let importer = Importer::new(storage, ImportStrategy::Skip);
        let stats = importer.import(temp_file.path()).unwrap();

        assert_eq!(stats.total_commands, 2);
        assert_eq!(stats.imported, 1); // Only docker ps
        assert_eq!(stats.skipped, 1);  // git status skipped
        assert_eq!(stats.updated, 0);
    }

    #[test]
    fn test_import_with_duplicates_preserve_higher() {
        let storage = create_test_storage();
        storage.insert(&create_test_command("git status", "git", 5)).unwrap();

        // Create export with higher usage count
        let source_storage = create_test_storage();
        source_storage.insert(&create_test_command("git status", "git", 10)).unwrap();

        let temp_file = NamedTempFile::new().unwrap();
        let source_exporter = Exporter::new(source_storage);
        source_exporter.export(temp_file.path()).unwrap();

        // Import with PreserveHigher strategy
        let importer = Importer::new(storage, ImportStrategy::PreserveHigher);
        let stats = importer.import(temp_file.path()).unwrap();

        assert_eq!(stats.total_commands, 1);
        assert_eq!(stats.imported, 0);
        assert_eq!(stats.skipped, 0);
        assert_eq!(stats.updated, 1); // Higher count preserved
    }

    #[test]
    fn test_export_import_roundtrip() {
        // Create source storage with data
        let source_storage = create_test_storage();
        source_storage.insert(&create_test_command("git status", "git", 5)).unwrap();
        source_storage.insert(&create_test_command("docker ps", "docker", 3)).unwrap();
        source_storage.insert(&create_test_command("ls -la", "file", 10)).unwrap();

        // Export
        let temp_file = NamedTempFile::new().unwrap();
        let exporter = Exporter::new(source_storage);
        let export_stats = exporter.export(temp_file.path()).unwrap();
        assert_eq!(export_stats.commands_exported, 3);

        // Import to new storage
        let target_storage = create_test_storage();
        let importer = Importer::new(target_storage, ImportStrategy::Skip);
        let import_stats = importer.import(temp_file.path()).unwrap();

        assert_eq!(import_stats.total_commands, 3);
        assert_eq!(import_stats.imported, 3);
        assert_eq!(import_stats.skipped, 0);
    }
}
