/// Storage layer for command history using SQLite
use crate::error::Result;
use crate::models::{CategoryStats, CommandRecord, OrderBy, SearchQuery, Stats};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

/// SQLite-based storage for command history
pub struct Storage {
    conn: Connection,
}

impl Storage {
    /// Create a new storage instance, initializing the database if needed
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let path = db_path.as_ref();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;
        
        // Enable WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
        
        let mut storage = Self { conn };
        storage.initialize_schema()?;
        
        Ok(storage)
    }

    /// Initialize the database schema
    fn initialize_schema(&mut self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS commands (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                command TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                exit_code INTEGER NOT NULL,
                duration_ms INTEGER NOT NULL,
                working_dir TEXT NOT NULL,
                category TEXT NOT NULL,
                usage_count INTEGER NOT NULL DEFAULT 1,
                last_used TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_timestamp ON commands(timestamp DESC);
            CREATE INDEX IF NOT EXISTS idx_category ON commands(category);
            CREATE INDEX IF NOT EXISTS idx_usage ON commands(usage_count DESC);
            CREATE INDEX IF NOT EXISTS idx_command ON commands(command);
            CREATE INDEX IF NOT EXISTS idx_exit_code ON commands(exit_code);

            -- Full-text search virtual table
            CREATE VIRTUAL TABLE IF NOT EXISTS commands_fts USING fts5(
                command,
                content='commands',
                content_rowid='id'
            );

            -- Triggers to keep FTS table in sync
            CREATE TRIGGER IF NOT EXISTS commands_ai AFTER INSERT ON commands BEGIN
                INSERT INTO commands_fts(rowid, command) VALUES (new.id, new.command);
            END;

            CREATE TRIGGER IF NOT EXISTS commands_ad AFTER DELETE ON commands BEGIN
                INSERT INTO commands_fts(commands_fts, rowid, command) 
                VALUES('delete', old.id, old.command);
            END;

            CREATE TRIGGER IF NOT EXISTS commands_au AFTER UPDATE ON commands BEGIN
                INSERT INTO commands_fts(commands_fts, rowid, command) 
                VALUES('delete', old.id, old.command);
                INSERT INTO commands_fts(rowid, command) VALUES (new.id, new.command);
            END;
            "#,
        )?;

        Ok(())
    }

    /// Insert a new command record
    pub fn insert(&self, cmd: &CommandRecord) -> Result<i64> {
        let timestamp_str = cmd.timestamp.to_rfc3339();
        let last_used_str = cmd.last_used.to_rfc3339();

        self.conn.execute(
            r#"
            INSERT INTO commands (command, timestamp, exit_code, duration_ms, 
                                 working_dir, category, usage_count, last_used)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                cmd.command,
                timestamp_str,
                cmd.exit_code,
                cmd.duration_ms,
                cmd.working_dir,
                cmd.category,
                cmd.usage_count,
                last_used_str,
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Find a duplicate command (same command text and working directory)
    pub fn find_duplicate(&self, command: &str, working_dir: &str) -> Result<Option<CommandRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, command, timestamp, exit_code, duration_ms, working_dir, 
                    category, usage_count, last_used
             FROM commands
             WHERE command = ?1 AND working_dir = ?2
             LIMIT 1",
        )?;

        let record = stmt
            .query_row(params![command, working_dir], |row| {
                Ok(CommandRecord {
                    id: Some(row.get(0)?),
                    command: row.get(1)?,
                    timestamp: row.get::<_, String>(2)?.parse().unwrap(),
                    exit_code: row.get(3)?,
                    duration_ms: row.get(4)?,
                    working_dir: row.get(5)?,
                    category: row.get(6)?,
                    usage_count: row.get(7)?,
                    last_used: row.get::<_, String>(8)?.parse().unwrap(),
                })
            })
            .optional()?;

        Ok(record)
    }

    /// Increment usage count for an existing command
    pub fn increment_usage(&self, id: i64) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "UPDATE commands SET usage_count = usage_count + 1, last_used = ?1 WHERE id = ?2",
            params![now, id],
        )?;

        Ok(())
    }

    /// Search commands with various filters
    pub fn search(&self, query: &SearchQuery) -> Result<Vec<CommandRecord>> {
        let mut sql = String::from(
            "SELECT id, command, timestamp, exit_code, duration_ms, working_dir,
                    category, usage_count, last_used
             FROM commands
             WHERE 1=1",
        );

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        // Add category filter
        if let Some(ref category) = query.category {
            sql.push_str(" AND category = ?");
            params.push(Box::new(category.clone()));
        }

        // Add success filter
        if let Some(success_only) = query.success_only {
            if success_only {
                sql.push_str(" AND exit_code = 0");
            } else {
                sql.push_str(" AND exit_code != 0");
            }
        }

        // Add text search if provided
        if let Some(ref text) = query.text {
            sql.push_str(" AND id IN (SELECT rowid FROM commands_fts WHERE command MATCH ?)");
            params.push(Box::new(text.clone()));
        }

        // Add ordering
        match query.order_by {
            OrderBy::Timestamp => sql.push_str(" ORDER BY timestamp DESC"),
            OrderBy::UsageCount | OrderBy::Relevance => {
                // For both usage count and relevance, order by usage count and timestamp
                // TODO: In the future, could use FTS5 rank() for better relevance scoring
                sql.push_str(" ORDER BY usage_count DESC, timestamp DESC");
            }
        }

        sql.push_str(&format!(" LIMIT {}", query.limit));

        let mut stmt = self.conn.prepare(&sql)?;
        
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        
        let records = stmt
            .query_map(param_refs.as_slice(), |row| {
                Ok(CommandRecord {
                    id: Some(row.get(0)?),
                    command: row.get(1)?,
                    timestamp: row.get::<_, String>(2)?.parse().unwrap(),
                    exit_code: row.get(3)?,
                    duration_ms: row.get(4)?,
                    working_dir: row.get(5)?,
                    category: row.get(6)?,
                    usage_count: row.get(7)?,
                    last_used: row.get::<_, String>(8)?.parse().unwrap(),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(records)
    }

    /// Get the most recent N commands
    pub fn get_recent(&self, limit: usize) -> Result<Vec<CommandRecord>> {
        let query = SearchQuery {
            text: None,
            category: None,
            success_only: None,
            limit,
            order_by: OrderBy::Timestamp,
        };

        self.search(&query)
    }

    /// Get the most frequently used commands
    pub fn get_top(&self, limit: usize) -> Result<Vec<CommandRecord>> {
        let query = SearchQuery {
            text: None,
            category: None,
            success_only: None,
            limit,
            order_by: OrderBy::UsageCount,
        };

        self.search(&query)
    }

    /// Get all commands in a specific category
    pub fn get_by_category(&self, category: &str, limit: usize) -> Result<Vec<CommandRecord>> {
        let query = SearchQuery {
            text: None,
            category: Some(category.to_string()),
            success_only: None,
            limit,
            order_by: OrderBy::UsageCount,
        };

        self.search(&query)
    }

    /// Get statistics about the command history
    pub fn get_stats(&self) -> Result<Stats> {
        // Total commands
        let total_commands: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM commands", [], |row| row.get(0))?;

        // Successful commands
        let successful_commands: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM commands WHERE exit_code = 0",
            [],
            |row| row.get(0),
        )?;

        // Failed commands
        let failed_commands = total_commands - successful_commands;

        // Commands by category
        let mut stmt = self.conn.prepare(
            "SELECT category, COUNT(*) as count FROM commands 
             GROUP BY category ORDER BY count DESC",
        )?;

        let by_category = stmt
            .query_map([], |row| {
                Ok(CategoryStats {
                    category: row.get(0)?,
                    count: row.get(1)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        // Oldest command timestamp
        let oldest_command: Option<String> = self
            .conn
            .query_row(
                "SELECT timestamp FROM commands ORDER BY timestamp ASC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?;

        // Newest command timestamp
        let newest_command: Option<String> = self
            .conn
            .query_row(
                "SELECT timestamp FROM commands ORDER BY timestamp DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?;

        Ok(Stats {
            total_commands,
            successful_commands,
            failed_commands,
            by_category,
            oldest_command: oldest_command.and_then(|s| s.parse().ok()),
            newest_command: newest_command.and_then(|s| s.parse().ok()),
        })
    }

    /// Get all commands (for export)
    pub fn get_all(&self) -> Result<Vec<CommandRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, command, timestamp, exit_code, duration_ms, working_dir,
                    category, usage_count, last_used
             FROM commands
             ORDER BY timestamp ASC",
        )?;

        let records = stmt
            .query_map([], |row| {
                Ok(CommandRecord {
                    id: Some(row.get(0)?),
                    command: row.get(1)?,
                    timestamp: row.get::<_, String>(2)?.parse().unwrap(),
                    exit_code: row.get(3)?,
                    duration_ms: row.get(4)?,
                    working_dir: row.get(5)?,
                    category: row.get(6)?,
                    usage_count: row.get(7)?,
                    last_used: row.get::<_, String>(8)?.parse().unwrap(),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(records)
    }

    /// Get total number of commands
    pub fn count(&self) -> Result<usize> {
        let count: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM commands", [], |row| row.get(0))?;
        Ok(count)
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

    fn create_test_command(command: &str, category: &str, exit_code: i32) -> CommandRecord {
        CommandRecord::new(
            command.to_string(),
            Utc::now(),
            exit_code,
            100,
            "/tmp".to_string(),
            category.to_string(),
        )
    }

    #[test]
    fn test_storage_creation() {
        let storage = create_test_storage();
        assert_eq!(storage.count().unwrap(), 0);
    }

    #[test]
    fn test_insert_command() {
        let storage = create_test_storage();
        let cmd = create_test_command("git status", "git", 0);

        let id = storage.insert(&cmd).unwrap();
        assert!(id > 0);
        assert_eq!(storage.count().unwrap(), 1);
    }

    #[test]
    fn test_find_duplicate() {
        let storage = create_test_storage();
        let cmd = create_test_command("git status", "git", 0);

        storage.insert(&cmd).unwrap();

        let duplicate = storage.find_duplicate("git status", "/tmp").unwrap();
        assert!(duplicate.is_some());
        assert_eq!(duplicate.unwrap().command, "git status");

        let not_found = storage.find_duplicate("git commit", "/tmp").unwrap();
        assert!(not_found.is_none());
    }

    #[test]
    fn test_increment_usage() {
        let storage = create_test_storage();
        let cmd = create_test_command("ls", "file", 0);

        let id = storage.insert(&cmd).unwrap();
        storage.increment_usage(id).unwrap();

        let records = storage.get_all().unwrap();
        assert_eq!(records[0].usage_count, 2);
    }

    #[test]
    fn test_search_by_category() {
        let storage = create_test_storage();
        
        storage.insert(&create_test_command("git status", "git", 0)).unwrap();
        storage.insert(&create_test_command("git commit", "git", 0)).unwrap();
        storage.insert(&create_test_command("docker ps", "docker", 0)).unwrap();

        let git_commands = storage.get_by_category("git", 10).unwrap();
        assert_eq!(git_commands.len(), 2);
    }

    #[test]
    fn test_get_stats() {
        let storage = create_test_storage();
        
        storage.insert(&create_test_command("success1", "git", 0)).unwrap();
        storage.insert(&create_test_command("success2", "docker", 0)).unwrap();
        storage.insert(&create_test_command("failure", "git", 1)).unwrap();

        let stats = storage.get_stats().unwrap();
        assert_eq!(stats.total_commands, 3);
        assert_eq!(stats.successful_commands, 2);
        assert_eq!(stats.failed_commands, 1);
        assert_eq!(stats.success_rate(), 66.66666666666666);
    }
}
