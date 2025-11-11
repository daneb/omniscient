# Technical Design Document

## System Architecture

### High-Level Overview

```
┌─────────────────────────────────────────────────────────────┐
│                         User Shell (Zsh)                     │
└────────────────┬───────────────────────────┬─────────────────┘
                 │                           │
          preexec hook                 precmd hook
                 │                           │
                 ▼                           ▼
         ┌───────────────┐          ┌───────────────┐
         │ Start Timer   │          │ Capture Cmd   │
         └───────────────┘          └───────┬───────┘
                                            │
                                            ▼
                              ┌─────────────────────────┐
                              │  omniscient capture     │
                              │  (Rust Binary)          │
                              └──────────┬──────────────┘
                                         │
                    ┌────────────────────┼────────────────────┐
                    │                    │                    │
                    ▼                    ▼                    ▼
            ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
            │  Redaction   │    │ Categorizer  │    │   Storage    │
            │   Engine     │    │   Engine     │    │   Layer      │
            └──────────────┘    └──────────────┘    └──────┬───────┘
                                                            │
                                                            ▼
                                                    ┌──────────────┐
                                                    │   SQLite     │
                                                    │   Database   │
                                                    └──────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    CLI Commands (User)                       │
│  search | recent | top | category | stats | export | import │
└────────────────────────┬─────────────────────────────────────┘
                         │
                         ▼
              ┌──────────────────────┐
              │  omniscient CLI      │
              │  (Rust Binary)       │
              └──────────┬───────────┘
                         │
                         ▼
                  ┌──────────────┐
                  │  Query       │
                  │  Engine      │
                  └──────┬───────┘
                         │
                         ▼
                  ┌──────────────┐
                  │   SQLite     │
                  │   Database   │
                  └──────────────┘
```

## Core Components

### 1. Shell Hook Integration

**File**: `src/shell.rs`

**Responsibility**: Generate shell-specific hook code

```rust
pub struct ShellHook {
    shell_type: ShellType,
}

pub enum ShellType {
    Zsh,
    // Future: Bash, Fish, PowerShell
}

impl ShellHook {
    pub fn generate(&self) -> String {
        // Generate shell-specific hook code
    }
}
```

**Zsh Hook Implementation**:
```zsh
# Timing
_omniscient_preexec() {
    export _OMNISCIENT_START=$EPOCHREALTIME
}

# Capture
_omniscient_precmd() {
    local exit_code=$?
    local cmd=$(fc -ln -1 | sed 's/^[[:space:]]*//')
    
    if [[ -n "$_OMNISCIENT_START" ]]; then
        local end=$EPOCHREALTIME
        local duration=$(( (end - _OMNISCIENT_START) * 1000 ))
        omniscient capture --exit-code "$exit_code" --duration "$duration" "$cmd" &
        unset _OMNISCIENT_START
    fi
}

precmd_functions+=(_omniscient_precmd)
preexec_functions+=(_omniscient_preexec)
```

**Key Design Decisions**:
- Asynchronous capture (background process with `&`)
- Trim leading whitespace from command
- Only capture if timer was set
- Clean up timer after capture

### 2. Capture Engine

**File**: `src/capture.rs`

**Responsibility**: Process incoming commands and store them

```rust
pub struct CaptureCommand {
    command: String,
    exit_code: i32,
    duration_ms: i64,
    working_dir: PathBuf,
    timestamp: DateTime<Utc>,
}

impl CaptureCommand {
    pub async fn execute(&self, storage: &Storage, config: &Config) -> Result<()> {
        // 1. Redact sensitive data
        let redacted_command = self.apply_redaction(config)?;
        
        // 2. Categorize
        let category = self.categorize()?;
        
        // 3. Check for duplicate
        if let Some(existing) = storage.find_duplicate(&redacted_command)? {
            // Update usage count and last_used
            storage.increment_usage(&existing.id)?;
        } else {
            // Insert new command
            storage.insert_command(CommandRecord {
                command: redacted_command,
                timestamp: self.timestamp,
                exit_code: self.exit_code,
                duration_ms: self.duration_ms,
                working_dir: self.working_dir.to_string_lossy().to_string(),
                category,
                usage_count: 1,
                last_used: self.timestamp,
            })?;
        }
        
        Ok(())
    }
}
```

**Performance Target**: < 10ms total execution time

### 3. Redaction Engine

**File**: `src/redact.rs`

**Responsibility**: Identify and redact sensitive information

```rust
pub struct RedactionEngine {
    patterns: Vec<Regex>,
}

impl RedactionEngine {
    pub fn new(patterns: Vec<String>) -> Result<Self> {
        let compiled = patterns
            .iter()
            .map(|p| Regex::new(p))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(Self { patterns: compiled })
    }
    
    pub fn should_redact(&self, command: &str) -> bool {
        self.patterns.iter().any(|p| p.is_match(command))
    }
    
    pub fn redact(&self, command: &str) -> String {
        if self.should_redact(command) {
            "[REDACTED]".to_string()
        } else {
            command.to_string()
        }
    }
}
```

**Default Patterns**:
- `password`
- `token`
- `secret`
- `api_key`
- `--password=`
- `-p ` (common password flag)

### 4. Categorization Engine

**File**: `src/category.rs`

**Responsibility**: Automatically categorize commands

```rust
pub struct Categorizer {
    rules: Vec<CategoryRule>,
}

struct CategoryRule {
    pattern: Regex,
    category: String,
}

impl Categorizer {
    pub fn categorize(&self, command: &str) -> String {
        // Extract first word (the actual command)
        let cmd = command.split_whitespace().next().unwrap_or("");
        
        // Check against rules
        for rule in &self.rules {
            if rule.pattern.is_match(cmd) {
                return rule.category.clone();
            }
        }
        
        "other".to_string()
    }
}
```

**Default Categories**:
- `git`: git, gh
- `docker`: docker, docker-compose, podman
- `package`: npm, yarn, cargo, pip, gem, apt, brew
- `file`: ls, cd, mkdir, rm, cp, mv, cat, grep
- `network`: curl, wget, ping, ssh, scp
- `build`: make, cmake, cargo, npm
- `database`: psql, mysql, sqlite3, mongo
- `kubernetes`: kubectl, k9s, helm
- `cloud`: aws, gcloud, az, terraform
- `other`: everything else

### 5. Storage Layer

**File**: `src/storage.rs`

**Responsibility**: Persist and retrieve command data

```rust
pub trait Storage {
    fn insert_command(&self, cmd: CommandRecord) -> Result<i64>;
    fn find_duplicate(&self, command: &str) -> Result<Option<CommandRecord>>;
    fn increment_usage(&self, id: i64) -> Result<()>;
    fn search(&self, query: &SearchQuery) -> Result<Vec<CommandRecord>>;
    fn get_stats(&self) -> Result<Stats>;
}

pub struct SqliteStorage {
    conn: Connection,
}

pub struct SearchQuery {
    pub text: Option<String>,
    pub category: Option<String>,
    pub limit: usize,
    pub order_by: OrderBy,
}

pub enum OrderBy {
    Timestamp,
    UsageCount,
    Relevance,
}
```

**SQLite Schema**:
```sql
CREATE TABLE IF NOT EXISTS commands (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    command TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    exit_code INTEGER NOT NULL,
    duration_ms INTEGER NOT NULL,
    working_dir TEXT NOT NULL,
    category TEXT NOT NULL,
    usage_count INTEGER NOT NULL DEFAULT 1,
    last_used TEXT NOT NULL,
    
    -- Full-text search
    UNIQUE(command, working_dir)
);

CREATE INDEX idx_timestamp ON commands(timestamp DESC);
CREATE INDEX idx_category ON commands(category);
CREATE INDEX idx_usage ON commands(usage_count DESC);
CREATE INDEX idx_command ON commands(command);

-- Full-text search virtual table
CREATE VIRTUAL TABLE commands_fts USING fts5(
    command,
    content='commands',
    content_rowid='id'
);
```

**Key Design Decisions**:
- Use FTS5 for fast full-text search
- Unique constraint on (command, working_dir) for duplicate detection
- Indexes on common query patterns
- Store timestamps as ISO 8601 strings

### 6. Search Engine

**File**: `src/search.rs`

**Responsibility**: Query and rank commands

```rust
pub struct SearchEngine {
    storage: Box<dyn Storage>,
}

impl SearchEngine {
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<CommandResult>> {
        // 1. Full-text search
        let mut results = self.storage.search(&SearchQuery {
            text: Some(query.to_string()),
            category: None,
            limit: limit * 2, // Get more for ranking
            order_by: OrderBy::Relevance,
        })?;
        
        // 2. Rank by relevance + usage
        results.sort_by(|a, b| {
            let score_a = self.calculate_score(a, query);
            let score_b = self.calculate_score(b, query);
            score_b.partial_cmp(&score_a).unwrap()
        });
        
        // 3. Limit results
        results.truncate(limit);
        
        Ok(results.into_iter().map(|r| r.into()).collect())
    }
    
    fn calculate_score(&self, record: &CommandRecord, query: &str) -> f64 {
        let mut score = 0.0;
        
        // Exact match bonus
        if record.command.contains(query) {
            score += 10.0;
        }
        
        // Usage frequency (logarithmic)
        score += (record.usage_count as f64).ln();
        
        // Recency (decay over time)
        let age_days = (Utc::now() - record.timestamp).num_days();
        score += 1.0 / (1.0 + age_days as f64 / 30.0);
        
        score
    }
}
```

**Ranking Algorithm**:
1. **Relevance**: Full-text search score (SQLite FTS5)
2. **Usage**: Logarithmic scaling of usage_count
3. **Recency**: Time decay (newer = higher score)
4. **Exact match**: Bonus for exact substring match

### 7. Export/Import

**File**: `src/export.rs`

**Responsibility**: Serialize/deserialize command history

```rust
#[derive(Serialize, Deserialize)]
pub struct ExportFormat {
    version: String,
    exported_at: DateTime<Utc>,
    total_commands: usize,
    commands: Vec<CommandRecord>,
}

pub struct Exporter {
    storage: Box<dyn Storage>,
}

impl Exporter {
    pub fn export(&self, path: &Path) -> Result<()> {
        let commands = self.storage.search(&SearchQuery {
            text: None,
            category: None,
            limit: usize::MAX,
            order_by: OrderBy::Timestamp,
        })?;
        
        let export = ExportFormat {
            version: env!("CARGO_PKG_VERSION").to_string(),
            exported_at: Utc::now(),
            total_commands: commands.len(),
            commands,
        };
        
        let json = serde_json::to_string_pretty(&export)?;
        fs::write(path, json)?;
        
        Ok(())
    }
    
    pub fn import(&self, path: &Path) -> Result<ImportStats> {
        let json = fs::read_to_string(path)?;
        let export: ExportFormat = serde_json::from_str(&json)?;
        
        let mut stats = ImportStats::default();
        
        for cmd in export.commands {
            match self.storage.find_duplicate(&cmd.command) {
                Ok(Some(existing)) => {
                    // Merge: take highest usage count
                    if cmd.usage_count > existing.usage_count {
                        self.storage.increment_usage(&existing.id)?;
                    }
                    stats.duplicates += 1;
                }
                Ok(None) => {
                    self.storage.insert_command(cmd)?;
                    stats.imported += 1;
                }
                Err(e) => {
                    stats.errors += 1;
                    eprintln!("Error importing command: {}", e);
                }
            }
        }
        
        Ok(stats)
    }
}
```

**Export Format** (JSON):
```json
{
  "version": "1.0.0",
  "exported_at": "2025-11-10T14:32:45Z",
  "total_commands": 1234,
  "commands": [
    {
      "id": 1,
      "command": "git status",
      "timestamp": "2025-11-10T14:30:12Z",
      "exit_code": 0,
      "duration_ms": 45,
      "working_dir": "/home/user/project",
      "category": "git",
      "usage_count": 89,
      "last_used": "2025-11-10T14:30:12Z"
    }
  ]
}
```

### 8. Configuration

**File**: `src/config.rs`

**Responsibility**: Load and manage configuration

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub storage: StorageConfig,
    pub privacy: PrivacyConfig,
    pub capture: CaptureConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageConfig {
    pub r#type: StorageType,
    pub path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StorageType {
    #[serde(rename = "sqlite")]
    Sqlite,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrivacyConfig {
    pub redact_patterns: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CaptureConfig {
    pub min_duration_ms: i64,
    pub max_history_size: usize,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            return Ok(Self::default());
        }
        
        let contents = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;
        
        Ok(config)
    }
    
    fn config_path() -> Result<PathBuf> {
        let home = env::var("HOME")?;
        Ok(PathBuf::from(home).join(".omniscient").join("config.toml"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            storage: StorageConfig {
                r#type: StorageType::Sqlite,
                path: PathBuf::from("~/.omniscient/history.db"),
            },
            privacy: PrivacyConfig {
                redact_patterns: vec![
                    "password".to_string(),
                    "token".to_string(),
                    "secret".to_string(),
                    "api_key".to_string(),
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
```

## CLI Interface

**File**: `src/main.rs`

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "omniscient")]
#[command(about = "CLI command history tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize shell integration
    Init,
    
    /// Capture a command (internal use by shell hook)
    Capture {
        #[arg(long)]
        exit_code: i32,
        
        #[arg(long)]
        duration: i64,
        
        command: String,
    },
    
    /// Search command history
    Search {
        query: String,
        
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    
    /// Show recent commands
    Recent {
        #[arg(default_value = "20")]
        n: usize,
    },
    
    /// Show most frequently used commands
    Top {
        #[arg(default_value = "10")]
        n: usize,
    },
    
    /// Filter by category
    Category {
        name: String,
        
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    
    /// Show statistics
    Stats,
    
    /// Export command history
    Export {
        #[arg(default_value = "history.json")]
        file: PathBuf,
    },
    
    /// Import command history
    Import {
        file: PathBuf,
    },
    
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,
    
    /// Edit configuration file
    Edit,
    
    /// Reset to defaults
    Reset,
}
```

## Data Flow Examples

### Capturing a Command

```
User types: git commit -m "feat: add search"
Press Enter
↓
preexec hook: _OMNISCIENT_START = 1699632765.123
↓
Shell executes: git commit -m "feat: add search"
Exit code: 0
↓
precmd hook:
  - exit_code = 0
  - cmd = "git commit -m \"feat: add search\""
  - duration = 234ms
  - working_dir = /home/user/project
↓
Execute (async): omniscient capture --exit-code 0 --duration 234 "git commit..."
↓
Capture Process:
  1. Check redaction: No patterns match
  2. Categorize: "git" (matches git command)
  3. Check duplicate: Not found
  4. Insert into SQLite:
     - command: git commit -m "feat: add search"
     - timestamp: 2025-11-10T14:32:45Z
     - exit_code: 0
     - duration_ms: 234
     - working_dir: /home/user/project
     - category: git
     - usage_count: 1
     - last_used: 2025-11-10T14:32:45Z
```

### Searching Commands

```
User types: omniscient search "docker build"
↓
Search Process:
  1. Query SQLite FTS5: MATCH 'docker build'
  2. Get 40 results (limit * 2)
  3. Rank results:
     - "docker build -t myapp ." (exact match, 45 uses) = 15.2 score
     - "docker build --no-cache ." (exact match, 12 uses) = 12.5 score
     - "docker-compose build" (partial, 89 uses) = 8.7 score
  4. Sort by score DESC
  5. Truncate to 20 results
↓
Display:
[2025-11-10 14:32:45] [✓] docker build -t myapp .
[2025-11-10 12:15:30] [✗] docker build --no-cache .
[2025-11-09 16:45:12] [✓] docker-compose build
...
```

## Performance Considerations

### Optimization Strategies

1. **Async Capture**: Run in background to avoid blocking shell
2. **SQLite Indexes**: Optimized for common query patterns
3. **FTS5**: Fast full-text search for large datasets
4. **Batch Imports**: Use transactions for importing large histories
5. **Memory-Mapped I/O**: Let SQLite handle caching efficiently

### Benchmarks (Target)

| Operation | Target | Method |
|-----------|--------|--------|
| Capture | < 10ms | Async background process |
| Search (1k cmds) | < 50ms | FTS5 + indexes |
| Search (100k cmds) | < 100ms | FTS5 + indexes |
| Export (100k cmds) | < 5s | Stream to file |
| Import (100k cmds) | < 10s | Batch insert with transaction |

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum OmniscientError {
    #[error("Storage error: {0}")]
    Storage(#[from] rusqlite::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Redaction error: {0}")]
    Redaction(String),
}

pub type Result<T> = std::result::Result<T, OmniscientError>;
```

### Error Handling Strategy

- **Capture Errors**: Log but don't interrupt shell (fail silently)
- **CLI Errors**: Display user-friendly message and exit with code
- **Storage Errors**: Attempt recovery, fallback to read-only mode
- **Config Errors**: Use defaults, warn user

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redaction() {
        let engine = RedactionEngine::new(vec!["password".to_string()]).unwrap();
        assert!(engine.should_redact("export PASSWORD=secret"));
        assert!(!engine.should_redact("git status"));
    }

    #[test]
    fn test_categorization() {
        let categorizer = Categorizer::default();
        assert_eq!(categorizer.categorize("git status"), "git");
        assert_eq!(categorizer.categorize("docker ps"), "docker");
        assert_eq!(categorizer.categorize("unknown_cmd"), "other");
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_capture_and_search() {
        // Create temp database
        // Capture a command
        // Search for it
        // Verify results
    }
    
    #[test]
    fn test_export_import() {
        // Create database with commands
        // Export to JSON
        // Create new database
        // Import from JSON
        // Verify all commands present
    }
}
```

## Security Considerations

### File Permissions

```rust
#[cfg(unix)]
fn set_secure_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o600); // rw-------
    fs::set_permissions(path, perms)?;
    Ok(())
}
```

### Redaction by Default

- Always enabled unless explicitly disabled
- Fail-safe: if pattern matching fails, redact the command
- Never log redacted commands to debug output

## Deployment

### Cross-Platform Builds

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

### Release Checklist

- [ ] Run all tests
- [ ] Build for all platforms
- [ ] Verify binary sizes < 10MB
- [ ] Test on clean machines
- [ ] Update version in Cargo.toml
- [ ] Tag release in Git
- [ ] Create GitHub release with binaries
- [ ] Update installation instructions

---

**Document Version**: 1.0  
**Last Updated**: 2025-11-10  
**Status**: Design Complete - Ready for Implementation