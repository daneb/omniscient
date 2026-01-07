use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for Omniscient operations
pub type Result<T> = std::result::Result<T, OmniscientError>;

/// Main error type for the Omniscient application
#[derive(Debug, Error)]
pub enum OmniscientError {
    /// Storage-related errors (SQLite)
    #[error("Storage error: {0}")]
    Storage(#[from] rusqlite::Error),

    /// I/O errors (file operations)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// TOML parsing errors
    #[error("TOML parsing error: {0}")]
    TomlParsing(#[from] toml::de::Error),

    /// Redaction pattern errors
    #[error("Redaction error: {0}")]
    Redaction(String),

    /// Database initialization errors
    #[error("Database initialization failed: {0}")]
    DatabaseInit(String),

    /// Command capture errors
    #[error("Failed to capture command: {0}")]
    Capture(String),

    /// Export/import errors
    #[error("Export/import error: {0}")]
    ExportImport(String),

    /// Shell integration errors
    #[error("Shell integration error: {0}")]
    Shell(String),

    /// Path-related errors
    #[error("Invalid path: {}", .0.display())]
    InvalidPath(PathBuf),

    /// Home directory not found
    #[error("Could not determine home directory")]
    NoHomeDir,

    /// Generic error for edge cases
    #[error("{0}")]
    Other(String),
}

impl OmniscientError {
    /// Create a config error with a custom message
    pub fn config<S: Into<String>>(msg: S) -> Self {
        Self::Config(msg.into())
    }

    /// Create a redaction error with a custom message
    pub fn redaction<S: Into<String>>(msg: S) -> Self {
        Self::Redaction(msg.into())
    }

    /// Create a capture error with a custom message
    pub fn capture<S: Into<String>>(msg: S) -> Self {
        Self::Capture(msg.into())
    }

    /// Create a shell integration error with a custom message
    pub fn shell<S: Into<String>>(msg: S) -> Self {
        Self::Shell(msg.into())
    }

    /// Create a generic error with a custom message
    pub fn other<S: Into<String>>(msg: S) -> Self {
        Self::Other(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = OmniscientError::config("test config error");
        assert_eq!(err.to_string(), "Configuration error: test config error");

        let err = OmniscientError::capture("failed to parse command");
        assert_eq!(
            err.to_string(),
            "Failed to capture command: failed to parse command"
        );
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = OmniscientError::from(io_err);
        assert!(err.to_string().contains("IO error"));
    }

    #[test]
    fn test_result_type() {
        fn returns_result() -> Result<i32> {
            Ok(42)
        }

        assert_eq!(returns_result().unwrap(), 42);
    }
}
