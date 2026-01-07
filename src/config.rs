/// Configuration management for Omniscient
use crate::error::{OmniscientError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub storage: StorageConfig,
    pub privacy: PrivacyConfig,
    pub capture: CaptureConfig,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage type (currently only "sqlite" is supported)
    #[serde(rename = "type")]
    pub storage_type: String,

    /// Path to the database file
    pub path: String,
}

/// Privacy and redaction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// List of regex patterns to redact
    pub redact_patterns: Vec<String>,

    /// Whether redaction is enabled
    pub enabled: bool,
}

/// Capture behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConfig {
    /// Minimum command duration to capture (ms)
    pub min_duration_ms: i64,

    /// Maximum number of commands to keep in history
    pub max_history_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            storage: StorageConfig {
                storage_type: "sqlite".to_string(),
                path: "~/.omniscient/history.db".to_string(),
            },
            privacy: PrivacyConfig {
                redact_patterns: vec![
                    "password".to_string(),
                    "token".to_string(),
                    "secret".to_string(),
                    "api_key".to_string(),
                    "apikey".to_string(),
                ],
                enabled: true,
            },
            capture: CaptureConfig {
                min_duration_ms: 0,
                max_history_size: 100_000,
            },
        }
    }
}

impl Config {
    /// Load configuration from file, or create default if it doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let contents = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| OmniscientError::config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&config_path, toml_string)?;

        Ok(())
    }

    /// Get the path to the configuration file
    pub fn config_path() -> Result<PathBuf> {
        let omniscient_dir = Self::omniscient_dir()?;
        Ok(omniscient_dir.join("config.toml"))
    }

    /// Get the Omniscient data directory (~/.omniscient)
    pub fn omniscient_dir() -> Result<PathBuf> {
        let home = Self::home_dir()?;
        Ok(home.join(".omniscient"))
    }

    /// Get the user's home directory
    pub fn home_dir() -> Result<PathBuf> {
        dirs::home_dir().ok_or(OmniscientError::NoHomeDir)
    }

    /// Expand tilde (~) in paths to home directory
    pub fn expand_path(&self, path: &str) -> Result<PathBuf> {
        if let Some(stripped) = path.strip_prefix("~/") {
            let home = Self::home_dir()?;
            Ok(home.join(stripped))
        } else if path == "~" {
            Self::home_dir()
        } else {
            Ok(PathBuf::from(path))
        }
    }

    /// Get the expanded database path
    pub fn database_path(&self) -> Result<PathBuf> {
        self.expand_path(&self.storage.path)
    }

    /// Ensure all required directories exist
    pub fn ensure_directories(&self) -> Result<()> {
        let omniscient_dir = Self::omniscient_dir()?;
        fs::create_dir_all(&omniscient_dir)?;

        // Ensure database directory exists
        if let Some(db_parent) = self.database_path()?.parent() {
            fs::create_dir_all(db_parent)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert_eq!(config.storage.storage_type, "sqlite");
        assert_eq!(config.storage.path, "~/.omniscient/history.db");
        assert!(config.privacy.enabled);
        assert!(!config.privacy.redact_patterns.is_empty());
        assert_eq!(config.capture.min_duration_ms, 0);
        assert_eq!(config.capture.max_history_size, 100_000);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_string = toml::to_string(&config).unwrap();

        assert!(toml_string.contains("[storage]"));
        assert!(toml_string.contains("[privacy]"));
        assert!(toml_string.contains("[capture]"));
        assert!(toml_string.contains("password"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_string = r#"
            [storage]
            type = "sqlite"
            path = "~/.omniscient/history.db"

            [privacy]
            redact_patterns = ["password", "token"]
            enabled = true

            [capture]
            min_duration_ms = 100
            max_history_size = 50000
        "#;

        let config: Config = toml::from_str(toml_string).unwrap();

        assert_eq!(config.storage.storage_type, "sqlite");
        assert_eq!(config.privacy.redact_patterns.len(), 2);
        assert_eq!(config.capture.min_duration_ms, 100);
        assert_eq!(config.capture.max_history_size, 50_000);
    }

    #[test]
    fn test_expand_path_with_tilde() {
        let config = Config::default();

        let expanded = config.expand_path("~/test/path").unwrap();
        assert!(!expanded.to_string_lossy().contains('~'));
        assert!(expanded.to_string_lossy().ends_with("test/path"));
    }

    #[test]
    fn test_expand_path_without_tilde() {
        let config = Config::default();

        let expanded = config.expand_path("/absolute/path").unwrap();
        assert_eq!(expanded, PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_database_path_expansion() {
        let config = Config::default();
        let db_path = config.database_path().unwrap();

        assert!(!db_path.to_string_lossy().contains('~'));
        assert!(db_path
            .to_string_lossy()
            .ends_with(".omniscient/history.db"));
    }
}
