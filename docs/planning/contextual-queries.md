# Contextual Queries - Path-Based Command History

## Feature Overview

**Version**: v1.2.0
**Status**: 🟡 Planning
**Created**: 2026-02-17

### Vision

Enable developers to retrieve command history that's relevant to their current working directory or project context. Instead of searching through thousands of commands globally, users can instantly see "what commands did I run in this folder?"

### Problem Statement

Currently, Omniscient captures `working_dir` for every command but doesn't expose it as a search filter. Developers often need to recall:
- "What commands did I run in this specific project?"
- "How did I configure this repository's build process?"
- "What deployment commands are used for this service?"

While global search is powerful, it returns commands from all contexts, making it harder to find relevant results when you need project-specific history.

---

## Core Principles Alignment

- **Lean & Minimal**: Reuses existing infrastructure (`working_dir` already stored)
- **Simple**: Natural CLI interface (`omniscient here`)
- **Performant**: Indexed queries, no new capture overhead
- **Human-First**: Matches developer mental model ("what did I do here?")

---

## Functional Requirements

### 1. Path-Based Filtering

#### Exact Match Mode (Default)
- Return commands executed in the exact current directory
- Use case: "What did I run in this specific subfolder?"
- Query: `working_dir = '/Users/dane/projects/myapp/src'`

#### Recursive Mode
- Return commands from current directory and all subdirectories
- Use case: "What have I done anywhere in this project?"
- Query: `working_dir LIKE '/Users/dane/projects/myapp%'`
- Flag: `--recursive` or `-r`

#### Absolute Path Support
- Allow specifying a path explicitly instead of using `pwd`
- Use case: "Show me commands from a different project without navigating there"
- Flag: `--dir <path>` or `-d <path>`

---

### 2. CLI Interface

#### New Subcommand: `here`

```bash
# Show all commands run in current directory (exact match)
omniscient here

# Show commands recursively (current dir + subdirs)
omniscient here --recursive
omniscient here -r

# Limit results
omniscient here --limit 50
omniscient here -l 50

# Combine with text search
omniscient here --search "git"
omniscient here -s "docker"

# Show only successful commands
omniscient here --success-only

# Different directory
omniscient here --dir /path/to/project
omniscient here -d ~/projects/myapp

# Order by usage count instead of recency
omniscient here --order usage
```

#### Enhanced Existing Commands

Add `--dir` and `--recursive` flags to existing subcommands:

```bash
# Search within a specific path
omniscient search "git" --dir ~/projects/myapp --recursive

# Recent commands in a project
omniscient recent --dir ~/projects/myapp -r

# Top commands in current directory
omniscient top --recursive
```

---

### 3. Output Format

Display path context clearly:

```
Showing commands in: /Users/dane/projects/omniscient (recursive)
Found 47 commands

[2026-02-17 10:23:45] [✓] cargo build --release
  Dir: /Users/dane/projects/omniscient
  Category: package | Used: 12 times

[2026-02-17 10:15:32] [✓] git commit -m "Add contextual queries"
  Dir: /Users/dane/projects/omniscient/.git
  Category: git | Used: 1 time

[2026-02-17 09:58:12] [✓] cargo test
  Dir: /Users/dane/projects/omniscient
  Category: package | Used: 45 times
```

---

## Technical Design

### 1. Data Model Changes

**File**: `src/models.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: Option<String>,
    pub category: Option<String>,
    pub success_only: bool,
    pub limit: usize,
    pub order_by: OrderBy,

    // NEW FIELDS
    pub working_dir: Option<String>,  // Path to filter by
    pub recursive: bool,               // Include subdirectories?
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            text: None,
            category: None,
            success_only: false,
            limit: 20,
            order_by: OrderBy::Timestamp,
            working_dir: None,  // NEW
            recursive: false,    // NEW
        }
    }
}
```

---

### 2. Storage Layer Changes

**File**: `src/storage.rs`

#### Add Performance Index

```rust
pub fn init_schema(&self) -> Result<()> {
    // ... existing schema ...

    // NEW: Index on working_dir for fast filtering
    self.conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_working_dir ON commands(working_dir)",
        [],
    )?;

    Ok(())
}
```

#### Update Search Method

```rust
pub fn search(&self, query: &SearchQuery) -> Result<Vec<CommandRecord>> {
    let mut sql = String::from("SELECT * FROM commands WHERE 1=1");
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

    // Existing filters (text, category, success_only)...

    // NEW: Working directory filter
    if let Some(dir) = &query.working_dir {
        if query.recursive {
            sql.push_str(" AND working_dir LIKE ?");
            params.push(Box::new(format!("{}%", dir)));
        } else {
            sql.push_str(" AND working_dir = ?");
            params.push(Box::new(dir.clone()));
        }
    }

    // ... rest of existing logic
}
```

---

### 3. CLI Changes

**File**: `src/main.rs`

#### New Subcommand

```rust
#[derive(Parser)]
#[command(name = "omniscient")]
enum Commands {
    // ... existing commands ...

    /// Show commands run in current (or specified) directory
    Here {
        /// Include subdirectories recursively
        #[arg(short, long)]
        recursive: bool,

        /// Specific directory to query (defaults to current)
        #[arg(short, long)]
        dir: Option<String>,

        /// Filter by text search
        #[arg(short, long)]
        search: Option<String>,

        /// Only show successful commands (exit code 0)
        #[arg(long)]
        success_only: bool,

        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,

        /// Order by: timestamp, usage, relevance
        #[arg(long, default_value = "timestamp")]
        order: String,
    },

    // ... rest of commands ...
}
```

#### Enhanced Existing Commands

```rust
Search {
    query: String,
    #[arg(short, long, default_value = "20")]
    limit: usize,

    // NEW FLAGS
    #[arg(short, long)]
    dir: Option<String>,

    #[arg(short, long)]
    recursive: bool,
}

Recent {
    count: Option<usize>,

    // NEW FLAGS
    #[arg(short, long)]
    dir: Option<String>,

    #[arg(short, long)]
    recursive: bool,
}

Top {
    count: Option<usize>,

    // NEW FLAGS
    #[arg(short, long)]
    dir: Option<String>,

    #[arg(short, long)]
    recursive: bool,
}
```

---

### 4. Path Handling

**Helper Functions** (add to `src/main.rs` or new `src/path.rs`):

```rust
use std::env;
use std::path::{Path, PathBuf};

/// Get the directory to query (from --dir flag or current directory)
fn resolve_query_dir(dir_arg: Option<String>) -> Result<String> {
    match dir_arg {
        Some(path) => {
            let canonical = Path::new(&path)
                .canonicalize()
                .map_err(|e| OmniscientError::InvalidPath(path.clone()))?;
            Ok(canonical.to_string_lossy().to_string())
        }
        None => {
            let cwd = env::current_dir()
                .map_err(|e| OmniscientError::Io(e))?;
            Ok(cwd.to_string_lossy().to_string())
        }
    }
}

/// Normalize path for consistent matching
/// - Expand ~ to home directory
/// - Resolve . and ..
/// - Remove trailing slashes
fn normalize_path(path: &str) -> Result<String> {
    let expanded = if path.starts_with("~") {
        path.replace("~", &dirs::home_dir()
            .ok_or(OmniscientError::Config("Cannot determine home directory".into()))?
            .to_string_lossy())
    } else {
        path.to_string()
    };

    let canonical = Path::new(&expanded)
        .canonicalize()
        .map_err(|_| OmniscientError::InvalidPath(path.to_string()))?;

    Ok(canonical.to_string_lossy().to_string())
}
```

---

### 5. Migration Handling

**Database Migration** (for adding index):

Since this is a non-breaking change (adding an index, not modifying schema), no migration is strictly required. The index will be created on first run after upgrade via `init_schema()`.

**Version Check** (optional, for defensive programming):

```rust
// In init_schema(), check if index exists before creating
let has_index: bool = self.conn.query_row(
    "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name='idx_working_dir'",
    [],
    |row| row.get(0)
)?;

if !has_index {
    self.conn.execute(
        "CREATE INDEX idx_working_dir ON commands(working_dir)",
        [],
    )?;
}
```

---

## Implementation Plan

### Phase 1: Core Functionality (3-4 hours)

#### Task 1.1: Data Model
- [ ] Add `working_dir` and `recursive` fields to `SearchQuery` in `src/models.rs`
- [ ] Update `Default` impl
- [ ] Add unit tests for new fields

**Files Modified**: `src/models.rs`

#### Task 1.2: Storage Layer
- [ ] Add `idx_working_dir` index creation to `init_schema()`
- [ ] Update `search()` method to handle `working_dir` and `recursive` filters
- [ ] Test exact match: `WHERE working_dir = ?`
- [ ] Test recursive match: `WHERE working_dir LIKE ?%`
- [ ] Add integration tests for path filtering

**Files Modified**: `src/storage.rs`

#### Task 1.3: Path Utilities
- [ ] Create `resolve_query_dir()` helper
- [ ] Create `normalize_path()` helper (handles ~, relative paths)
- [ ] Add tests for path normalization edge cases
- [ ] Handle errors (invalid paths, permissions)

**Files Modified**: `src/main.rs` (or new `src/path.rs`)

---

### Phase 2: CLI Interface (2-3 hours)

#### Task 2.1: New `here` Command
- [ ] Add `Here` variant to `Commands` enum
- [ ] Implement command handler
- [ ] Get current working directory or use `--dir`
- [ ] Build `SearchQuery` with path filter
- [ ] Format output with path context header
- [ ] Add examples to help text

**Files Modified**: `src/main.rs`

#### Task 2.2: Enhance Existing Commands
- [ ] Add `--dir` and `--recursive` flags to `search`
- [ ] Add `--dir` and `--recursive` flags to `recent`
- [ ] Add `--dir` and `--recursive` flags to `top`
- [ ] Update help documentation

**Files Modified**: `src/main.rs`

---

### Phase 3: Testing & Polish (2-3 hours)

#### Task 3.1: Integration Tests
- [ ] Test `omniscient here` with exact match
- [ ] Test `omniscient here --recursive`
- [ ] Test `omniscient here --dir /path/to/project`
- [ ] Test `omniscient search "text" --dir /path --recursive`
- [ ] Test with non-existent directory (error handling)
- [ ] Test with symlinks
- [ ] Test with relative paths (., .., ~/path)

**Files Created**: `tests/contextual_queries_test.rs`

#### Task 3.2: Performance Testing
- [ ] Benchmark queries with `idx_working_dir` index
- [ ] Compare recursive vs exact match performance
- [ ] Test with large datasets (10k, 100k, 1M commands)
- [ ] Ensure <100ms query time for typical use cases

#### Task 3.3: Documentation
- [ ] Update README.md with `here` command examples
- [ ] Add to DOCUMENTATION.md (contextual queries section)
- [ ] Update CHANGELOG.md
- [ ] Add usage examples to `--help` output

**Files Modified**: `README.md`, `DOCUMENTATION.md`, `CHANGELOG.md`

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_path_filter() {
        let storage = setup_test_db();
        storage.insert_command("git status", "/home/user/project");
        storage.insert_command("ls -la", "/home/user/project/src");

        let query = SearchQuery {
            working_dir: Some("/home/user/project".into()),
            recursive: false,
            ..Default::default()
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].command, "git status");
    }

    #[test]
    fn test_recursive_path_filter() {
        let storage = setup_test_db();
        storage.insert_command("git status", "/home/user/project");
        storage.insert_command("cargo build", "/home/user/project/crate1");
        storage.insert_command("npm install", "/home/user/other");

        let query = SearchQuery {
            working_dir: Some("/home/user/project".into()),
            recursive: true,
            ..Default::default()
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_path_normalization() {
        assert_eq!(normalize_path("~/projects"), "/home/user/projects");
        assert_eq!(normalize_path("./src"), "/current/working/dir/src");
        assert_eq!(normalize_path("/path/to/../dir"), "/path/dir");
    }
}
```

### Integration Tests

```bash
# Setup test environment with known commands
omniscient capture "git status" --working-dir /projects/myapp
omniscient capture "cargo build" --working-dir /projects/myapp/src
omniscient capture "npm install" --working-dir /projects/other

# Test exact match
cd /projects/myapp
omniscient here
# Expected: Only "git status"

# Test recursive
cd /projects/myapp
omniscient here --recursive
# Expected: "git status" and "cargo build"

# Test --dir flag
omniscient here --dir /projects/other
# Expected: "npm install"
```

---

## Edge Cases & Error Handling

### Path Edge Cases
1. **Relative paths**: Resolve to absolute before querying
2. **Symlinks**: Canonicalize to real path
3. **Trailing slashes**: Normalize (`/path/` → `/path`)
4. **Case sensitivity**: Match OS behavior (case-insensitive on macOS, sensitive on Linux)
5. **Non-existent paths**: Return error with helpful message

### Error Messages

```rust
// Invalid path
if !Path::new(&dir).exists() {
    return Err(OmniscientError::InvalidPath(
        format!("Directory does not exist: {}", dir)
    ));
}

// Permission denied
Err(OmniscientError::Io(io_error)) => {
    eprintln!("Cannot access directory: {}", dir);
}
```

---

## Performance Considerations

### Index Performance

**Before** (without `idx_working_dir`):
- Full table scan for path filtering
- O(n) complexity, ~500ms for 100k rows

**After** (with index):
- Index seek + range scan
- O(log n) complexity, ~5ms for 100k rows

### Query Optimization

```sql
-- Exact match (uses index directly)
EXPLAIN QUERY PLAN
SELECT * FROM commands WHERE working_dir = '/path/to/project';
-- Result: SEARCH commands USING INDEX idx_working_dir

-- Recursive match (index range scan)
EXPLAIN QUERY PLAN
SELECT * FROM commands WHERE working_dir LIKE '/path/to/project%';
-- Result: SEARCH commands USING INDEX idx_working_dir (uses prefix scan)
```

---

## User Experience Scenarios

### Scenario 1: "What did I do in this project?"

```bash
cd ~/projects/omniscient
omniscient here -r

# Output:
# Showing commands in: /Users/dane/projects/omniscient (recursive)
# Found 142 commands
#
# [2026-02-17 10:23:45] [✓] cargo build --release
# [2026-02-17 09:15:32] [✓] git commit -m "Add feature"
# [2026-02-16 14:42:10] [✓] cargo test
# ...
```

### Scenario 2: "Deployment commands for this service"

```bash
cd ~/services/api-gateway
omniscient here -s "deploy" -r

# Output filtered to deployment-related commands in this service
```

### Scenario 3: "Compare commands across projects"

```bash
omniscient here --dir ~/projects/rust-app -r -l 10 > rust-commands.txt
omniscient here --dir ~/projects/node-app -r -l 10 > node-commands.txt
diff rust-commands.txt node-commands.txt
```

---

## Migration Path for Users

### No Breaking Changes

This is a **purely additive** feature:
- Existing commands work unchanged
- New flags are optional
- `working_dir` data already exists in DB
- Index creation is automatic and non-breaking

### Upgrade Experience

```bash
# User upgrades to v1.2.0
cargo install omniscient --force

# On first run, index is created automatically
omniscient here
# "Creating index on working_dir... done (0.2s)"
# "Showing commands in: /Users/dane/current/dir"
```

---

## Future Enhancements (Out of Scope for v1.2)

1. **Project Detection**: Automatically detect project roots (git, cargo, package.json)
2. **Smart Context**: Combine path + category (e.g., "git commands in this project")
3. **Directory Aliases**: `omniscient alias myapp ~/projects/myapp`
4. **Path Autocomplete**: Shell completion for `--dir` flag
5. **Stats by Path**: `omniscient stats --dir ~/projects/myapp`

---

## Success Metrics

### Functionality
- [ ] `omniscient here` returns commands from current directory
- [ ] `omniscient here -r` includes subdirectories
- [ ] `omniscient here --dir /path` works from any location
- [ ] Path filtering works with all existing flags (search, success-only, etc.)
- [ ] Performance: <10ms for exact match, <50ms for recursive on 100k rows

### Quality
- [ ] Zero test failures
- [ ] Zero clippy warnings
- [ ] Documentation updated
- [ ] All edge cases handled with clear error messages

### User Experience
- [ ] Intuitive command naming (`here` is memorable)
- [ ] Help text clearly explains recursive vs exact match
- [ ] Output shows path context clearly
- [ ] Works with shell completion

---

## Dependencies

### No New External Dependencies Required

All functionality uses existing crates:
- `rusqlite` - for SQL index and LIKE queries
- `std::env` - for `current_dir()`
- `std::path` - for path normalization
- `clap` - for new CLI flags

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Path normalization differs across OSes | High | Use `Path::canonicalize()` consistently |
| Performance degradation on recursive queries | Medium | Index on `working_dir`, benchmark before release |
| Confusion between exact vs recursive | Low | Clear help text, good defaults |
| Symlink edge cases | Low | Document behavior, canonicalize paths |

---

## Timeline Estimate

- **Phase 1**: 3-4 hours (data model + storage + path utils)
- **Phase 2**: 2-3 hours (CLI interface)
- **Phase 3**: 2-3 hours (testing + docs)

**Total**: 7-10 hours of development time

**Target Release**: v1.2.0

---

## Open Questions

1. **Command naming**: Is `here` the best name, or `context`, `local`, `pwd`?
2. **Default behavior**: Should `here` default to recursive or exact match?
3. **Performance threshold**: What's acceptable query time for 1M commands?
4. **Path display**: Show relative or absolute paths in output?

---

## References

- Current implementation: [src/storage.rs:71-89](src/storage.rs#L71-L89) (existing `working_dir` capture)
- Similar tools: `atuin` (has workspace awareness), `mcfly` (has directory-based ranking)
- SQL LIKE optimization: https://www.sqlite.org/optoverview.html#like_opt
