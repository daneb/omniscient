/// Data models for Omniscient
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single command execution record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandRecord {
    /// Unique identifier (database primary key)
    pub id: Option<i64>,
    
    /// The command text as executed
    pub command: String,
    
    /// When the command was executed
    pub timestamp: DateTime<Utc>,
    
    /// Exit code (0 = success, non-zero = failure)
    pub exit_code: i32,
    
    /// How long the command took to execute (milliseconds)
    pub duration_ms: i64,
    
    /// Working directory where command was executed
    pub working_dir: String,
    
    /// Automatically assigned category (git, docker, etc.)
    pub category: String,
    
    /// Number of times this command has been executed
    pub usage_count: i32,
    
    /// Timestamp of most recent execution
    pub last_used: DateTime<Utc>,
}

impl CommandRecord {
    /// Create a new command record (before database insertion)
    pub fn new(
        command: String,
        timestamp: DateTime<Utc>,
        exit_code: i32,
        duration_ms: i64,
        working_dir: String,
        category: String,
    ) -> Self {
        Self {
            id: None, // Will be assigned by database
            command,
            timestamp,
            exit_code,
            duration_ms,
            working_dir,
            category,
            usage_count: 1,
            last_used: timestamp,
        }
    }

    /// Check if the command was successful (exit code 0)
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }

    /// Get a display-friendly status indicator
    pub fn status_symbol(&self) -> &str {
        if self.is_success() {
            "✓"
        } else {
            "✗"
        }
    }

    /// Format duration for human-readable display
    pub fn duration_display(&self) -> String {
        if self.duration_ms < 1000 {
            format!("{}ms", self.duration_ms)
        } else if self.duration_ms < 60_000 {
            format!("{:.1}s", self.duration_ms as f64 / 1000.0)
        } else {
            let minutes = self.duration_ms / 60_000;
            let seconds = (self.duration_ms % 60_000) / 1000;
            format!("{}m{}s", minutes, seconds)
        }
    }
}

/// Statistics about command history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    /// Total number of commands in history
    pub total_commands: usize,
    
    /// Number of successful commands (exit code 0)
    pub successful_commands: usize,
    
    /// Number of failed commands (exit code != 0)
    pub failed_commands: usize,
    
    /// Commands grouped by category with counts
    pub by_category: Vec<CategoryStats>,
    
    /// Date of oldest command
    pub oldest_command: Option<DateTime<Utc>>,
    
    /// Date of newest command
    pub newest_command: Option<DateTime<Utc>>,
}

impl Stats {
    /// Calculate success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_commands == 0 {
            0.0
        } else {
            (self.successful_commands as f64 / self.total_commands as f64) * 100.0
        }
    }
}

/// Statistics for a single category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    pub category: String,
    pub count: usize,
}

/// Query parameters for searching commands
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// Text to search for (optional)
    pub text: Option<String>,
    
    /// Filter by category (optional)
    pub category: Option<String>,
    
    /// Filter by success/failure (optional)
    pub success_only: Option<bool>,
    
    /// Maximum number of results
    pub limit: usize,
    
    /// How to order results
    pub order_by: OrderBy,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            text: None,
            category: None,
            success_only: None,
            limit: 20,
            order_by: OrderBy::Timestamp,
        }
    }
}

/// Ordering options for search results
#[derive(Debug, Clone, Copy)]
pub enum OrderBy {
    /// Most recent first
    Timestamp,
    
    /// Most frequently used first
    UsageCount,
    
    /// Best relevance match first (for text searches)
    Relevance,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_command_record_creation() {
        let cmd = CommandRecord::new(
            "git status".to_string(),
            Utc::now(),
            0,
            45,
            "/home/user/project".to_string(),
            "git".to_string(),
        );

        assert_eq!(cmd.command, "git status");
        assert_eq!(cmd.exit_code, 0);
        assert_eq!(cmd.duration_ms, 45);
        assert_eq!(cmd.category, "git");
        assert_eq!(cmd.usage_count, 1);
        assert!(cmd.is_success());
    }

    #[test]
    fn test_status_symbol() {
        let success = CommandRecord::new(
            "ls".to_string(),
            Utc::now(),
            0,
            10,
            "/tmp".to_string(),
            "file".to_string(),
        );
        assert_eq!(success.status_symbol(), "✓");

        let failure = CommandRecord::new(
            "ls /nonexistent".to_string(),
            Utc::now(),
            1,
            10,
            "/tmp".to_string(),
            "file".to_string(),
        );
        assert_eq!(failure.status_symbol(), "✗");
    }

    #[test]
    fn test_duration_display() {
        let cmd = CommandRecord::new(
            "test".to_string(),
            Utc::now(),
            0,
            500,
            "/tmp".to_string(),
            "other".to_string(),
        );
        assert_eq!(cmd.duration_display(), "500ms");

        let cmd = CommandRecord::new(
            "test".to_string(),
            Utc::now(),
            0,
            2500,
            "/tmp".to_string(),
            "other".to_string(),
        );
        assert_eq!(cmd.duration_display(), "2.5s");

        let cmd = CommandRecord::new(
            "test".to_string(),
            Utc::now(),
            0,
            125000,
            "/tmp".to_string(),
            "other".to_string(),
        );
        assert_eq!(cmd.duration_display(), "2m5s");
    }

    #[test]
    fn test_stats_success_rate() {
        let stats = Stats {
            total_commands: 100,
            successful_commands: 85,
            failed_commands: 15,
            by_category: vec![],
            oldest_command: None,
            newest_command: None,
        };

        assert_eq!(stats.success_rate(), 85.0);
    }

    #[test]
    fn test_search_query_default() {
        let query = SearchQuery::default();
        assert_eq!(query.limit, 20);
        assert!(query.text.is_none());
        assert!(query.category.is_none());
    }
}
