/// Storage layer for command history using SQLite
use crate::error::Result;
use crate::models::{CategoryStats, CommandRecord, OrderBy, SearchQuery, Stats};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::time::Duration;

/// SQLite-based storage for command history
pub struct Storage {
    conn: Connection,
}

/// How long a writer waits for a held lock before giving up.
///
/// Longer than rusqlite's 5s default, and set here so the guarantee is this
/// crate's rather than a dependency's.
const BUSY_TIMEOUT: Duration = Duration::from_secs(10);

impl Storage {
    /// The busy timeout this connection will wait for a held write lock.
    pub fn busy_timeout_ms(&self) -> Result<i64> {
        Ok(self
            .conn
            .query_row("PRAGMA busy_timeout", [], |row| row.get(0))?)
    }

    /// Create a new storage instance, initializing the database if needed
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let path = db_path.as_ref();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;

        // Wait for a held write lock rather than failing instantly.
        //
        // rusqlite already defaults this to 5s, so this is not fixing a bug —
        // it states the guarantee explicitly rather than inheriting it from a
        // library default that could change, and lengthens it. Every terminal is
        // a separate process writing this one file and WAL permits one writer at
        // a time, so the wait is real; a capture that waits costs nothing
        // because it is backgrounded away from the prompt, and a capture that is
        // dropped costs a command forever.
        conn.busy_timeout(BUSY_TIMEOUT)?;

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
            CREATE INDEX IF NOT EXISTS idx_working_dir ON commands(working_dir);

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
    pub fn find_duplicate(
        &self,
        command: &str,
        working_dir: &str,
    ) -> Result<Option<CommandRecord>> {
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

    /// Sanitizes a query string for FTS5 search by wrapping it in quotes
    /// This treats the query as a literal phrase, preventing FTS5 syntax errors
    /// for special characters like dots, asterisks, etc.
    ///
    /// # Arguments
    /// * `query` - The raw search query from the user
    ///
    /// # Returns
    /// A sanitized query string safe for FTS5 MATCH clause
    ///
    /// # Examples
    /// ```
    /// use omniscient::Storage;
    /// # use tempfile::NamedTempFile;
    /// # let temp_file = NamedTempFile::new().unwrap();
    /// # let storage = Storage::new(temp_file.path()).unwrap();
    /// // The sanitize function is private, but here's how it works internally
    /// let query = "10.104.113.39";
    /// // Internally sanitized to: "\"10.104.113.39\""
    /// ```
    fn sanitize_fts5_query(query: &str) -> String {
        // Escape existing double quotes by doubling them (FTS5 standard)
        let escaped = query.replace("\"", "\"\"");

        // Wrap entire query in quotes for literal phrase search
        // This makes FTS5 treat all special characters as literals
        format!("\"{}\"", escaped)
    }

    /// Fallback search using SQL LIKE when FTS5 fails
    /// This is slower but handles any character combination
    ///
    /// # Arguments
    /// * `text` - The search text
    /// * `category` - Optional category filter
    /// * `success_only` - Optional success filter
    /// * `limit` - Maximum number of results
    ///
    /// # Returns
    /// Vector of matching command records
    fn search_with_like(&self, query: &SearchQuery, text: &str) -> Result<Vec<CommandRecord>> {
        let mut sql = String::from(
            "SELECT id, command, timestamp, exit_code, duration_ms, working_dir,
                    category, usage_count, last_used
             FROM commands
             WHERE command LIKE ?",
        );

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(format!("%{}%", text))];

        // Add category filter
        if let Some(ref cat) = &query.category {
            sql.push_str(" AND category = ?");
            params.push(Box::new(cat.clone()));
        }

        // Add success filter
        if let Some(success) = query.success_only {
            if success {
                sql.push_str(" AND exit_code = 0");
            } else {
                sql.push_str(" AND exit_code != 0");
            }
        }

        // Add working directory filter
        if let Some(ref dir) = &query.working_dir {
            if query.recursive {
                sql.push_str(" AND working_dir LIKE ?");
                params.push(Box::new(format!("{}%", dir)));
            } else {
                sql.push_str(" AND working_dir = ?");
                params.push(Box::new(dir.clone()));
            }
        }

        // Add ordering
        match query.order_by {
            OrderBy::Timestamp => sql.push_str(" ORDER BY timestamp DESC"),
            OrderBy::UsageCount => sql.push_str(" ORDER BY usage_count DESC, timestamp DESC"),
            OrderBy::Relevance => sql.push_str(
                " ORDER BY CAST(usage_count AS REAL) / ((julianday('now') - julianday(last_used)) * 24.0 + 1.0) DESC, usage_count DESC"
            ),
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

        // Add working directory filter
        if let Some(ref dir) = query.working_dir {
            if query.recursive {
                sql.push_str(" AND working_dir LIKE ?");
                params.push(Box::new(format!("{}%", dir)));
            } else {
                sql.push_str(" AND working_dir = ?");
                params.push(Box::new(dir.clone()));
            }
        }

        // Add text search if provided
        if let Some(ref text) = query.text {
            // Sanitize query for FTS5 to handle special characters
            let sanitized = Self::sanitize_fts5_query(text);
            sql.push_str(" AND id IN (SELECT rowid FROM commands_fts WHERE command MATCH ?)");
            params.push(Box::new(sanitized));
        }

        // Add ordering
        match query.order_by {
            OrderBy::Timestamp => sql.push_str(" ORDER BY timestamp DESC"),
            OrderBy::UsageCount => sql.push_str(" ORDER BY usage_count DESC, timestamp DESC"),
            OrderBy::Relevance => sql.push_str(
                " ORDER BY CAST(usage_count AS REAL) / ((julianday('now') - julianday(last_used)) * 24.0 + 1.0) DESC, usage_count DESC"
            ),
        }

        sql.push_str(&format!(" LIMIT {}", query.limit));

        // Try FTS5 search first, fall back to LIKE if it fails
        let stmt_result = self.conn.prepare(&sql);

        let records = match stmt_result {
            Ok(mut stmt) => {
                let param_refs: Vec<&dyn rusqlite::ToSql> =
                    params.iter().map(|p| p.as_ref()).collect();

                let rows_result = stmt.query_map(param_refs.as_slice(), |row| {
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
                });

                match rows_result {
                    Ok(rows) => rows.collect::<std::result::Result<Vec<_>, _>>()?,
                    Err(_) if query.text.is_some() => {
                        // FTS5 query failed, fall back to LIKE search
                        self.search_with_like(query, query.text.as_ref().unwrap())?
                    }
                    Err(e) => return Err(e.into()),
                }
            }
            Err(_) if query.text.is_some() => {
                // FTS5 prepare failed, fall back to LIKE search
                self.search_with_like(query, query.text.as_ref().unwrap())?
            }
            Err(e) => return Err(e.into()),
        };

        Ok(records)
    }

    /// Get the most recent N commands
    pub fn get_recent(
        &self,
        limit: usize,
        working_dir: Option<String>,
        recursive: bool,
    ) -> Result<Vec<CommandRecord>> {
        let query = SearchQuery {
            text: None,
            category: None,
            success_only: None,
            working_dir,
            recursive,
            limit,
            order_by: OrderBy::Timestamp,
        };

        self.search(&query)
    }

    /// Get the most frequently used commands
    ///
    /// Without a directory filter, the same command run in several working
    /// directories is one logical command: usage counts are summed, duration
    /// is averaged weighted by usage, and the most recent `last_used` wins.
    /// With a directory filter, rows are scoped to that directory as before
    /// and are not grouped across directories.
    pub fn get_top(
        &self,
        limit: usize,
        working_dir: Option<String>,
        recursive: bool,
    ) -> Result<Vec<CommandRecord>> {
        if working_dir.is_none() {
            return self.get_top_grouped(limit);
        }

        let query = SearchQuery {
            text: None,
            category: None,
            success_only: None,
            working_dir,
            recursive,
            limit,
            order_by: OrderBy::UsageCount,
        };

        self.search(&query)
    }

    /// Get the most frequently used commands, grouped by command text across
    /// all working directories.
    fn get_top_grouped(&self, limit: usize) -> Result<Vec<CommandRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT command,
                    MAX(category) AS category,
                    SUM(usage_count) AS total_usage,
                    MAX(last_used) AS last_used,
                    CAST(ROUND(SUM(duration_ms * usage_count) * 1.0 / SUM(usage_count)) AS INTEGER) AS avg_duration_ms
             FROM commands
             GROUP BY command
             ORDER BY total_usage DESC, last_used DESC
             LIMIT ?1",
        )?;

        let records = stmt
            .query_map(params![limit as i64], |row| {
                let last_used: String = row.get(3)?;
                Ok(CommandRecord {
                    id: None,
                    command: row.get(0)?,
                    timestamp: last_used.parse().unwrap(),
                    exit_code: 0,
                    duration_ms: row.get(4)?,
                    working_dir: String::new(),
                    category: row.get(1)?,
                    usage_count: row.get(2)?,
                    last_used: last_used.parse().unwrap(),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(records)
    }

    /// Get all commands in a specific category
    pub fn get_by_category(
        &self,
        category: &str,
        limit: usize,
        working_dir: Option<String>,
        recursive: bool,
    ) -> Result<Vec<CommandRecord>> {
        let query = SearchQuery {
            text: None,
            category: Some(category.to_string()),
            success_only: None,
            working_dir,
            recursive,
            limit,
            order_by: OrderBy::UsageCount,
        };

        self.search(&query)
    }

    /// Get statistics about the command history
    pub fn get_stats(&self) -> Result<Stats> {
        // Total commands
        let total_commands: usize =
            self.conn
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

        storage
            .insert(&create_test_command("git status", "git", 0))
            .unwrap();
        storage
            .insert(&create_test_command("git commit", "git", 0))
            .unwrap();
        storage
            .insert(&create_test_command("docker ps", "docker", 0))
            .unwrap();

        let git_commands = storage.get_by_category("git", 10, None, false).unwrap();
        assert_eq!(git_commands.len(), 2);
    }

    #[test]
    fn test_sanitize_fts5_query_simple() {
        let result = Storage::sanitize_fts5_query("hello world");
        assert_eq!(result, "\"hello world\"");
    }

    #[test]
    fn test_sanitize_fts5_query_with_dots() {
        let result = Storage::sanitize_fts5_query("10.104.113.39");
        assert_eq!(result, "\"10.104.113.39\"");
    }

    #[test]
    fn test_sanitize_fts5_query_with_quotes() {
        let result = Storage::sanitize_fts5_query("grep \"pattern\"");
        assert_eq!(result, "\"grep \"\"pattern\"\"\"");
    }

    #[test]
    fn test_sanitize_fts5_query_with_asterisk() {
        let result = Storage::sanitize_fts5_query("ls *.txt");
        assert_eq!(result, "\"ls *.txt\"");
    }

    #[test]
    fn test_sanitize_fts5_query_url() {
        let result = Storage::sanitize_fts5_query("https://example.com");
        assert_eq!(result, "\"https://example.com\"");
    }

    #[test]
    fn test_search_with_ip_address() {
        let storage = create_test_storage();

        // Insert a command with an IP address
        let record = create_test_command("ssh user@10.104.113.39", "network", 0);
        storage.insert(&record).unwrap();

        // Search for the IP address
        let query = SearchQuery {
            text: Some("10.104.113.39".to_string()),
            category: None,
            success_only: None,
            working_dir: None,
            recursive: false,
            limit: 10,
            order_by: OrderBy::Relevance,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].command.contains("10.104.113.39"));
    }

    #[test]
    fn test_search_with_url() {
        let storage = create_test_storage();

        let record = create_test_command("curl https://api.github.com/users/daneb", "network", 0);
        storage.insert(&record).unwrap();

        let query = SearchQuery {
            text: Some("api.github.com".to_string()),
            category: None,
            success_only: None,
            working_dir: None,
            recursive: false,
            limit: 10,
            order_by: OrderBy::Relevance,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_with_file_path() {
        let storage = create_test_storage();

        let record = create_test_command("cat ./config/settings.yaml", "file", 0);
        storage.insert(&record).unwrap();

        let query = SearchQuery {
            text: Some("./config/settings.yaml".to_string()),
            category: None,
            success_only: None,
            working_dir: None,
            recursive: false,
            limit: 10,
            order_by: OrderBy::Relevance,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_with_multiple_special_chars() {
        let storage = create_test_storage();

        let record = create_test_command("scp file.txt user@host.com:/path/to/dest", "network", 0);
        storage.insert(&record).unwrap();

        let query = SearchQuery {
            text: Some("user@host.com".to_string()),
            category: None,
            success_only: None,
            working_dir: None,
            recursive: false,
            limit: 10,
            order_by: OrderBy::Relevance,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_empty_query_still_works() {
        let storage = create_test_storage();

        let record = create_test_command("ls -la", "file", 0);
        storage.insert(&record).unwrap();

        // Search without text (should use other filters)
        let query = SearchQuery {
            text: None,
            category: Some("file".to_string()),
            success_only: None,
            working_dir: None,
            recursive: false,
            limit: 10,
            order_by: OrderBy::Timestamp,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_get_top_groups_same_command_across_directories() {
        let storage = create_test_storage();

        let mut cmd_a = create_test_command("gst", "other", 0);
        cmd_a.working_dir = "/tmp/a".to_string();
        cmd_a.duration_ms = 100;
        let id_a = storage.insert(&cmd_a).unwrap();
        for _ in 0..4 {
            storage.increment_usage(id_a).unwrap();
        }
        // usage_count is now 5 in /tmp/a

        let mut cmd_b = create_test_command("gst", "other", 0);
        cmd_b.working_dir = "/tmp/b".to_string();
        cmd_b.duration_ms = 300;
        let id_b = storage.insert(&cmd_b).unwrap();
        for _ in 0..2 {
            storage.increment_usage(id_b).unwrap();
        }
        // usage_count is now 3 in /tmp/b

        storage
            .insert(&create_test_command("ls", "file", 0))
            .unwrap();

        let top = storage.get_top(10, None, false).unwrap();

        assert_eq!(top.iter().filter(|c| c.command == "gst").count(), 1);

        let gst = top.iter().find(|c| c.command == "gst").unwrap();
        assert_eq!(gst.usage_count, 8);
        // Weighted by usage: (100 * 5 + 300 * 3) / 8 = 175
        assert_eq!(gst.duration_ms, 175);
    }

    #[test]
    fn test_get_top_with_dir_filter_is_not_grouped() {
        let storage = create_test_storage();

        let mut cmd_a = create_test_command("gst", "other", 0);
        cmd_a.working_dir = "/tmp/a".to_string();
        storage.insert(&cmd_a).unwrap();

        let mut cmd_b = create_test_command("gst", "other", 0);
        cmd_b.working_dir = "/tmp/b".to_string();
        storage.insert(&cmd_b).unwrap();

        let top = storage
            .get_top(10, Some("/tmp/a".to_string()), false)
            .unwrap();

        assert_eq!(top.len(), 1);
        assert_eq!(top[0].working_dir, "/tmp/a");
    }

    #[test]
    fn test_get_stats() {
        let storage = create_test_storage();

        storage
            .insert(&create_test_command("success1", "git", 0))
            .unwrap();
        storage
            .insert(&create_test_command("success2", "docker", 0))
            .unwrap();
        storage
            .insert(&create_test_command("failure", "git", 1))
            .unwrap();

        let stats = storage.get_stats().unwrap();
        assert_eq!(stats.total_commands, 3);
        assert_eq!(stats.successful_commands, 2);
        assert_eq!(stats.failed_commands, 1);
        assert_eq!(stats.success_rate(), 66.66666666666666);
    }
}
