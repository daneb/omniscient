/// Command capture functionality - integrates redaction, categorization, and storage
use crate::category::Categorizer;
use crate::config::Config;
use crate::error::Result;
use crate::models::CommandRecord;
use crate::redact::RedactionEngine;
use crate::storage::Storage;
use chrono::Utc;
use std::env;

/// Captures and stores a command execution
pub struct CommandCapture {
    storage: Storage,
    redactor: RedactionEngine,
    categorizer: Categorizer,
    config: Config,
}

impl CommandCapture {
    /// Create a new command capture instance
    pub fn new(config: Config) -> Result<Self> {
        let db_path = config.database_path()?;
        let storage = Storage::new(db_path)?;

        let redactor = RedactionEngine::new(
            config.privacy.redact_patterns.clone(),
            config.privacy.enabled,
        )?;

        let categorizer = Categorizer::new();

        Ok(Self {
            storage,
            redactor,
            categorizer,
            config,
        })
    }

    /// Capture a command and store it
    pub fn capture(&self, command: &str, exit_code: i32, duration_ms: i64) -> Result<()> {
        // Skip if command is empty or whitespace only
        let command = command.trim();
        if command.is_empty() {
            return Ok(());
        }

        // Skip if duration is below minimum threshold
        if duration_ms < self.config.capture.min_duration_ms {
            return Ok(());
        }

        // Check if command should be redacted
        let processed_command = self.redactor.redact(command);

        // If redacted, we don't want to store any information
        if processed_command == "[REDACTED]" {
            return Ok(());
        }

        // Get current working directory
        let working_dir = env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "/unknown".to_string());

        // Categorize the command
        let category = self.categorizer.categorize(&processed_command);

        // Check if this command already exists
        if let Some(existing) = self
            .storage
            .find_duplicate(&processed_command, &working_dir)?
        {
            // Update usage count
            self.storage.increment_usage(existing.id.unwrap())?;
        } else {
            // Create new command record
            let record = CommandRecord::new(
                processed_command,
                Utc::now(),
                exit_code,
                duration_ms,
                working_dir,
                category,
            );

            // Insert into storage
            self.storage.insert(&record)?;
        }

        Ok(())
    }

    /// Get statistics about captured commands
    pub fn stats(&self) -> Result<crate::models::Stats> {
        self.storage.get_stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_config() -> Config {
        let mut config = Config::default();
        let temp_file = NamedTempFile::new().unwrap();
        config.storage.path = temp_file.path().to_string_lossy().to_string();
        config
    }

    #[test]
    fn test_capture_creation() {
        let config = create_test_config();
        let capture = CommandCapture::new(config);
        assert!(capture.is_ok());
    }

    #[test]
    fn test_capture_simple_command() {
        let config = create_test_config();
        let capture = CommandCapture::new(config).unwrap();

        let result = capture.capture("git status", 0, 100);
        assert!(result.is_ok());

        let stats = capture.stats().unwrap();
        assert_eq!(stats.total_commands, 1);
    }

    #[test]
    fn test_capture_duplicate_command() {
        let config = create_test_config();
        let capture = CommandCapture::new(config).unwrap();

        // Capture same command twice
        capture.capture("git status", 0, 100).unwrap();
        capture.capture("git status", 0, 150).unwrap();

        let stats = capture.stats().unwrap();
        assert_eq!(stats.total_commands, 1); // Only one unique command

        // Verify usage count was incremented
        let commands = capture.storage.get_recent(10, None, false).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].usage_count, 2);
    }

    #[test]
    fn test_capture_redacted_command() {
        let config = create_test_config();
        let capture = CommandCapture::new(config).unwrap();

        // Command with "password" should be redacted and not stored
        capture.capture("export PASSWORD=secret", 0, 100).unwrap();

        let stats = capture.stats().unwrap();
        assert_eq!(stats.total_commands, 0); // Should not be stored
    }

    #[test]
    fn test_capture_categorization() {
        let config = create_test_config();
        let capture = CommandCapture::new(config).unwrap();

        capture.capture("git status", 0, 100).unwrap();
        capture.capture("docker ps", 0, 50).unwrap();
        capture.capture("npm install", 0, 2000).unwrap();

        let commands = capture.storage.get_recent(10, None, false).unwrap();
        assert_eq!(commands.len(), 3);

        // Check categories
        let git_cmd = commands.iter().find(|c| c.command == "git status").unwrap();
        assert_eq!(git_cmd.category, "git");

        let docker_cmd = commands.iter().find(|c| c.command == "docker ps").unwrap();
        assert_eq!(docker_cmd.category, "docker");

        let npm_cmd = commands
            .iter()
            .find(|c| c.command == "npm install")
            .unwrap();
        assert_eq!(npm_cmd.category, "package");
    }

    #[test]
    fn test_capture_empty_command() {
        let config = create_test_config();
        let capture = CommandCapture::new(config).unwrap();

        // Empty commands should be skipped
        capture.capture("", 0, 100).unwrap();
        capture.capture("   ", 0, 100).unwrap();

        let stats = capture.stats().unwrap();
        assert_eq!(stats.total_commands, 0);
    }

    #[test]
    fn test_capture_with_exit_codes() {
        let config = create_test_config();
        let capture = CommandCapture::new(config).unwrap();

        capture.capture("ls /existing", 0, 10).unwrap();
        capture.capture("ls /nonexistent", 1, 10).unwrap();

        let stats = capture.stats().unwrap();
        assert_eq!(stats.total_commands, 2);
        assert_eq!(stats.successful_commands, 1);
        assert_eq!(stats.failed_commands, 1);
    }

    #[test]
    fn test_capture_min_duration_filter() {
        let mut config = create_test_config();
        config.capture.min_duration_ms = 100; // Only capture commands > 100ms

        let capture = CommandCapture::new(config).unwrap();

        capture.capture("fast command", 0, 50).unwrap(); // Too fast
        capture.capture("slow command", 0, 200).unwrap(); // Should be captured

        let stats = capture.stats().unwrap();
        assert_eq!(stats.total_commands, 1);

        let commands = capture.storage.get_recent(10, None, false).unwrap();
        assert_eq!(commands[0].command, "slow command");
    }

    #[test]
    fn test_capture_disabled_redaction() {
        let mut config = create_test_config();
        config.privacy.enabled = false; // Disable redaction

        let capture = CommandCapture::new(config).unwrap();

        // Even with "password", it should be stored when redaction is disabled
        capture.capture("export PASSWORD=secret", 0, 100).unwrap();

        let stats = capture.stats().unwrap();
        assert_eq!(stats.total_commands, 1);

        let commands = capture.storage.get_recent(10, None, false).unwrap();
        assert_eq!(commands[0].command, "export PASSWORD=secret");
    }

    #[test]
    fn test_capture_different_working_dirs() {
        let config = create_test_config();
        let capture = CommandCapture::new(config).unwrap();

        // Same command in different directories should be treated as different
        // Note: In real usage, working_dir would change, but in tests it's the same
        // This test documents expected behavior
        capture.capture("ls", 0, 10).unwrap();
        capture.capture("ls", 0, 10).unwrap();

        // Should only have one entry (same command, same directory)
        let commands = capture.storage.get_recent(10, None, false).unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].usage_count, 2);
    }
}
