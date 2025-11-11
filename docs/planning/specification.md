# Omniscient - CLI Command History Tracker

## Project Overview

A cross-platform command history tracker that captures, categorizes, and preserves all CLI commands across shell sessions and machine reinstalls. Built with Rust for performance and reliability, following principles of simplicity and minimalism.

## Vision

Enable developers to maintain a complete, searchable history of their command-line interactions that survives machine migrations, with intelligent categorization and easy retrieval.

## Core Principles

- **Lean & Minimal**: No unnecessary features or complexity
- **Simple**: Easy to understand, install, and use
- **Performant**: Fast capture, fast retrieval
- **Human-First**: Designed for developer workflow

---

## Version 1.0 Scope

### Functional Requirements

#### 1. Command Capture
- **Integration Method**: Zsh shell hook (precmd)
- **Captured Data**:
  - Command text (full command as typed)
  - Timestamp (ISO 8601 format)
  - Exit code (0-255)
  - Execution duration (milliseconds)
  - Working directory (absolute path)
- **Success Criteria**: Exit code 0 = success, non-zero = failure

#### 2. Data Storage
- **Storage Format**: SQLite database (for querying capabilities) or structured JSON file
- **Location**: `~/.omniscient/` directory
  - Database: `~/.omniscient/history.db` or `~/.omniscient/history.json`
  - Config: `~/.omniscient/config.toml`
- **Schema Design** (if SQLite):
  ```sql
  CREATE TABLE commands (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      command TEXT NOT NULL,
      timestamp TEXT NOT NULL,
      exit_code INTEGER NOT NULL,
      duration_ms INTEGER NOT NULL,
      working_dir TEXT NOT NULL,
      category TEXT,
      usage_count INTEGER DEFAULT 1,
      last_used TEXT NOT NULL
  );
  
  CREATE INDEX idx_timestamp ON commands(timestamp);
  CREATE INDEX idx_command ON commands(command);
  CREATE INDEX idx_category ON commands(category);
  CREATE INDEX idx_usage ON commands(usage_count DESC);
  ```

#### 3. Categorization
- **Automatic Categories** (v1):
  - By command type (git, docker, npm, cargo, etc.)
  - Pattern matching on first word of command
  - Default category: "other"
- **Frequency Tracking**:
  - Increment `usage_count` for duplicate commands
  - Update `last_used` timestamp
  - Rank results by frequency in search

#### 4. Privacy & Security
- **Redaction Support**:
  - Command-line flag: `--redact <pattern>`
  - Config file option for permanent redaction patterns
  - Example: redact any command containing "password", "token", "secret"
  - Redacted commands stored as: `[REDACTED]`

#### 5. Search & Retrieval (CLI)
- **Search Commands**:
  - `omniscient search <query>` - Search by text
  - `omniscient recent [n]` - Show last n commands (default 20)
  - `omniscient stats` - Show usage statistics
  - `omniscient top [n]` - Most frequently used commands
  - `omniscient category <name>` - Filter by category
- **Output Format**: Plain text for easy copy/paste
  ```
  [2025-11-10 14:32:45] [✓] git commit -m "feat: add search"
  [2025-11-10 14:30:12] [✗] cargo build --release
  ```

#### 6. Sync & Portability
- **Export**: `omniscient export [file]` - Export to JSON
- **Import**: `omniscient import <file>` - Import from JSON, merge with existing
- **GitHub Sync**: Manual workflow
  1. Export to JSON
  2. Commit to private Git repository
  3. Clone on new machine
  4. Import from JSON
- **Re-initialization**: `omniscient init` - Set up on new machine

#### 7. Installation
- **Distribution**: Single binary executable
- **Installation Methods**:
  - Direct download from GitHub releases
  - `cargo install omniscient` (optional)
- **Setup**: `omniscient init` generates shell hook code to add to `~/.zshrc`

---

## Technical Architecture

### Technology Stack
- **Language**: Rust (2021 edition)
- **Database**: SQLite via `rusqlite` crate (or `serde_json` for file-based)
- **CLI Framework**: `clap` for argument parsing
- **Configuration**: `toml` format via `toml` crate
- **Date/Time**: `chrono` crate

### Key Components

#### 1. Shell Hook
```zsh
# Added to ~/.zshrc by `omniscient init`
_omniscient_precmd() {
    local exit_code=$?
    local cmd=$(fc -ln -1)
    local duration=$((EPOCHREALTIME - _omniscient_start))
    omniscient capture --exit-code $exit_code --duration $duration "$cmd"
}

_omniscient_preexec() {
    _omniscient_start=$EPOCHREALTIME
}

precmd_functions+=(_omniscient_precmd)
preexec_functions+=(_omniscient_preexec)
```

#### 2. CLI Commands Structure
```
omniscient
├── init           # Initialize shell integration
├── capture        # Internal: called by shell hook
├── search <query> # Search command history
├── recent [n]     # Show recent commands
├── top [n]        # Most frequent commands
├── category <cat> # Filter by category
├── stats          # Show statistics
├── export [file]  # Export to JSON
├── import <file>  # Import from JSON
└── config         # Manage configuration
```

#### 3. Configuration File (`~/.omniscient/config.toml`)
```toml
[storage]
type = "sqlite"  # or "json"
path = "~/.omniscient/history.db"

[privacy]
redact_patterns = ["password", "token", "secret", "api_key"]
enabled = true

[capture]
min_duration_ms = 0  # Don't capture commands faster than this
max_history_size = 100000  # Rotate after this many entries
```

### Data Flow

1. **Command Execution**:
   ```
   User types command → Zsh preexec hook (start timer)
   → Command executes → Zsh precmd hook (capture)
   → omniscient capture called → Data stored
   ```

2. **Command Storage**:
   ```
   Shell hook → omniscient capture
   → Parse command → Check redaction patterns
   → Extract category → Check for duplicate
   → Update usage_count OR insert new
   → Write to SQLite
   ```

3. **Command Retrieval**:
   ```
   User: omniscient search "git"
   → Query SQLite with pattern matching
   → Rank by usage_count + relevance
   → Format output → Display
   ```

---

## Success Metrics

### Performance Targets
- Command capture: < 10ms overhead
- Search response: < 100ms for 10,000+ commands
- Binary size: < 10MB
- Memory usage: < 50MB during operation

### Usability Goals
- Setup time: < 2 minutes
- Zero configuration required for basic use
- Intuitive command names
- Clear, parseable output

---

## Future Enhancements (Out of Scope for v1)

### Deferred Features
1. **Command Aliases Expansion**: Show both alias and expanded command
2. **Multi-line Command Handling**: Proper handling of multi-line commands and here-docs
3. **Collaborative Sharing**: Team command libraries, shared snippets
4. **AI-Powered Suggestions**: Context-aware command recommendations

### Potential v2 Features
- Support for Bash, Fish, PowerShell
- Web UI for browsing history
- Cloud sync options (beyond Git)
- Command execution with safety checks
- Integration with shell's Ctrl+R (reverse search)
- User-defined tags and notes
- Command templates and variables

---

## Development Phases

### Phase 1: Core Infrastructure (Week 1)
- [ ] Project setup (Cargo.toml, directory structure)
- [ ] SQLite schema and basic CRUD operations
- [ ] Configuration file handling
- [ ] Command categorization logic

### Phase 2: Capture Mechanism (Week 1-2)
- [ ] `omniscient capture` command
- [ ] Shell hook generation
- [ ] `omniscient init` setup command
- [ ] Redaction pattern matching

### Phase 3: Search & Retrieval (Week 2)
- [ ] `omniscient search` with ranking
- [ ] `omniscient recent` command
- [ ] `omniscient top` command
- [ ] `omniscient category` command
- [ ] `omniscient stats` command

### Phase 4: Sync & Portability (Week 2-3)
- [ ] Export to JSON
- [ ] Import from JSON with merge logic
- [ ] Duplicate detection

### Phase 5: Testing & Documentation (Week 3)
- [ ] Unit tests for core functions
- [ ] Integration tests
- [ ] README with installation instructions
- [ ] Usage examples

### Phase 6: Release (Week 3-4)
- [ ] Cross-platform builds (Linux, macOS, Windows)
- [ ] GitHub releases
- [ ] Installation scripts

---

## File Structure

```
omniscient/
├── Cargo.toml
├── README.md
├── SPECIFICATION.md (this file)
├── LICENSE
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── capture.rs           # Command capture logic
│   ├── storage.rs           # SQLite/file storage
│   ├── search.rs            # Search and retrieval
│   ├── category.rs          # Categorization logic
│   ├── config.rs            # Configuration handling
│   ├── export.rs            # Export/import functionality
│   ├── shell.rs             # Shell integration code generation
│   └── redact.rs            # Privacy/redaction logic
├── tests/
│   ├── integration_tests.rs
│   └── fixtures/
└── examples/
    └── shell_hooks/
        └── zsh_hook.sh
```

---

## Security Considerations

### Sensitive Data Handling
- Default redaction patterns for common secrets
- User-configurable redaction
- Local-only storage (no automatic cloud sync)
- Warnings about syncing to public repositories

### File Permissions
- Database file: 600 (read/write owner only)
- Config file: 600
- Directory: 700

---

## Testing Strategy

### Unit Tests
- Categorization logic
- Redaction pattern matching
- Command deduplication
- Export/import format validation

### Integration Tests
- End-to-end capture flow
- Search accuracy and ranking
- Import merging logic
- Shell hook generation

### Manual Testing
- Real-world Zsh integration
- Performance with large datasets (10k, 100k commands)
- Cross-platform compatibility

---

## Open Questions & Decisions Needed

### Storage Choice
**Decision Point**: SQLite vs JSON file

**SQLite Pros**:
- Efficient querying
- Indexing for fast search
- ACID compliance
- Well-tested

**JSON Pros**:
- Human-readable
- Git-friendly (easy diffing)
- No additional dependencies
- Simple implementation

**Recommendation**: Start with SQLite for v1, consider JSON export format as primary with SQLite as cache/index in v2.

### Duplicate Handling
**Question**: How to identify duplicate commands?
- Exact match only?
- Ignore leading/trailing whitespace?
- Case-sensitive or insensitive?

**Recommendation**: Exact match after trimming whitespace, case-sensitive.

---

## Success Criteria for v1 Release

- [ ] Successfully captures commands in Zsh
- [ ] Stores commands with all required metadata
- [ ] Categorizes commands automatically
- [ ] Search returns relevant results in < 100ms
- [ ] Export/import preserves all data
- [ ] Redaction works correctly
- [ ] Works on Linux and macOS
- [ ] Installation takes < 2 minutes
- [ ] Documentation is clear and complete
- [ ] Zero known security issues

---

## Next Steps

1. Review and approve this specification
2. Set up Rust project structure
3. Implement storage layer
4. Build capture mechanism
5. Develop search functionality
6. Create shell integration
7. Add export/import
8. Test and iterate
9. Document and release

---

**Document Version**: 1.0  
**Last Updated**: 2025-11-10  
**Author**: Dane Balia  
**Status**: Draft - Awaiting Approval