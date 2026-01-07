/// Omniscient - CLI command history tracker
///
/// This library provides the core functionality for tracking, storing,
/// and searching command-line history across sessions.
pub mod capture;
pub mod category;
pub mod config;
pub mod error;
pub mod export;
pub mod models;
pub mod redact;
pub mod shell;
pub mod storage;

// Re-export commonly used types
pub use capture::CommandCapture;
pub use category::Categorizer;
pub use config::Config;
pub use error::{OmniscientError, Result};
pub use export::{Exporter, ImportStrategy, Importer};
pub use models::{CommandRecord, OrderBy, SearchQuery, Stats};
pub use redact::RedactionEngine;
pub use shell::{ShellHook, ShellType};
pub use storage::Storage;
