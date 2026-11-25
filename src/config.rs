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

    /// Where capture failures are recorded.
    ///
    /// Beside the database rather than on stderr: the shell hook runs capture as
    /// `... &>/dev/null &!`, so anything printed is discarded. A failure that
    /// only prints is a failure that never happened.
    pub fn failure_log_path(&self) -> Result<PathBuf> {
        Ok(match self.database_path()?.parent() {
            Some(dir) => dir.join("capture-failures.log"),
            None => PathBuf::from("capture-failures.log"),
        })
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

    /// Serialises tests that point `HOME` at a temp dir; the environment is
    /// process-global and `Config::home_dir` reads it.
    static HOME_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Runs `f` with `HOME` set to a fresh temp dir, restoring it afterwards.
    fn with_temp_home<F: FnOnce(&std::path::Path)>(f: F) {
        let _guard = HOME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = tempfile::tempdir().unwrap();
        let old = std::env::var_os("HOME");
        std::env::set_var("HOME", tmp.path());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(tmp.path())));
        match old {
            Some(v) => std::env::set_var("HOME", v),
            None => std::env::remove_var("HOME"),
        }
        if let Err(e) = result {
            std::panic::resume_unwind(e);
        }
    }

    #[test]
    fn test_omniscient_dir_is_under_home() {
        with_temp_home(|home| {
            assert_eq!(Config::omniscient_dir().unwrap(), home.join(".omniscient"));
        });
    }

    #[test]
    fn test_ensure_directories_creates_db_and_failure_log_parents() {
        with_temp_home(|home| {
            let mut config = Config::default();
            config.storage.path = "~/nested/db/history.db".to_string();

            config.ensure_directories().unwrap();

            assert!(home.join(".omniscient").is_dir());
            assert!(config.database_path().unwrap().parent().unwrap().is_dir());
            assert!(config
                .failure_log_path()
                .unwrap()
                .parent()
                .unwrap()
                .is_dir());
        });
    }

    #[test]
    fn test_ensure_directories_succeeds_when_already_present() {
        with_temp_home(|_| {
            let config = Config::default();
            config.ensure_directories().unwrap();
            config.ensure_directories().unwrap();
        });
    }

    #[test]
    fn test_ensure_directories_with_absolute_db_path() {
        with_temp_home(|home| {
            let mut config = Config::default();
            let db = home.join("elsewhere").join("history.db");
            config.storage.path = db.to_string_lossy().into_owned();

            config.ensure_directories().unwrap();

            assert!(home.join("elsewhere").is_dir());
        });
    }

    #[test]
    fn test_failure_log_path_expands_tilde() {
        with_temp_home(|home| {
            let config = Config::default();
            assert_eq!(
                config.failure_log_path().unwrap(),
                home.join(".omniscient").join("capture-failures.log")
            );
        });
    }

    #[test]
    fn test_failure_log_path_keeps_absolute_path() {
        let mut config = Config::default();
        config.storage.path = "/var/data/history.db".to_string();
        assert_eq!(
            config.failure_log_path().unwrap(),
            PathBuf::from("/var/data/capture-failures.log")
        );
    }

    #[test]
    fn test_database_path_tilde_and_absolute() {
        with_temp_home(|home| {
            let mut config = Config::default();
            assert_eq!(
                config.database_path().unwrap(),
                home.join(".omniscient").join("history.db")
            );
            config.storage.path = "/var/data/history.db".to_string();
            assert_eq!(
                config.database_path().unwrap(),
                PathBuf::from("/var/data/history.db")
            );
        });
    }
}
