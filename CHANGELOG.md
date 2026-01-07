# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.1] - 2026-01-07

### Fixed
- **Search with Special Characters** - Fixed search failure when querying for commands containing special characters
  - IP addresses (e.g., `10.104.113.39`) now work correctly
  - URLs (e.g., `https://api.github.com`) are fully supported
  - File paths with dots (e.g., `./config/settings.yaml`) search successfully
  - Email addresses and other special character combinations work properly
  - Added automatic FTS5 query sanitization by wrapping queries in quotes
  - Implemented fallback to SQL LIKE search for edge cases where FTS5 still fails
  - Error: "fts5: syntax error near \".\"" is now resolved

### Added
- `sanitize_fts5_query()` helper function for safe FTS5 queries
- `search_with_like()` fallback method for robustness
- 7 new test cases for special character searches (98 total tests)
- Architecture Decision Record: [ADR-001: FTS5 Query Sanitization](docs/adr/ADR-001-fts5-query-sanitization.md)

### Changed
- All search queries now use exact phrase matching for better accuracy
- Search is now more robust with multi-layered fallback mechanisms

## [1.1.0] - 2025-11-12

### Added
- **Bash Support** - Full support for Bash shell alongside Zsh
  - Implemented `generate_bash()` hook generator
  - Added `--shell` flag to `omniscient init` command for explicit shell selection
  - Auto-detection of current shell via `$SHELL` environment variable
  - Integration with bash-preexec library for preexec/precmd hooks
  - Command timing using `date +%s%N` (nanosecond precision)
  - Silent background execution with `& + disown` (Bash equivalent of Zsh's `&!`)
- Added `ShellType::Bash` to shell type enum
- Created `examples/bash_hook.sh` reference file
- Added 6 new Bash-specific tests (81 total tests)
- Comprehensive Bash installation instructions in README
- Shell detection method `ShellHook::detect_shell()`

### Changed
- `omniscient init` now accepts optional `--shell <type>` parameter
- Updated README with multi-shell setup instructions
- Added multi-shell support to features list

### Documentation
- Added Bash setup guide to README with bash-preexec installation
- Created planning document for multi-shell support strategy
- Updated examples directory with Bash hook example

## [1.0.2] - 2025-11-11

### Fixed
- Fixed Zsh job notification spam after each command
- Shell hook now uses `&!` (background and disown) to prevent job control messages
- Commands now run silently in the background without "[2] + done" messages

## [1.0.1] - 2025-11-11

### Added
- Created `uninstall.sh` script for automated uninstallation
- Added uninstallation documentation to README
- Uninstaller creates backups before removing data

### Fixed
- Fixed floating-point duration bug in Zsh shell hook that caused capture to fail
- Duration calculation now correctly converts to integer milliseconds using `int()` function
- Fixed unused variable warning in export tests

## [1.0.0] - 2025-11-11

### Added

#### Core Infrastructure
- SQLite database storage with WAL mode for concurrency
- Full-text search using FTS5 virtual tables
- Comprehensive configuration system with TOML
- Type-safe error handling with thiserror
- Complete CRUD operations for command records

#### Capture Mechanism
- Automatic command capture via Zsh shell hooks
- Smart categorization of 80+ commands into 13 categories
- Privacy-first redaction engine with configurable patterns
- Duplicate detection and usage count tracking
- Background execution with zero shell impact
- Exit code and duration tracking

#### Search & Retrieval
- Full-text search with relevance ranking
- Recent commands display (ordered by timestamp)
- Top commands by usage frequency
- Category-based filtering
- Comprehensive statistics dashboard
- Time range analysis

#### Export/Import
- JSON export with versioning
- Three import strategies (Skip, UpdateUsage, PreserveHigher)
- Export/import statistics
- Round-trip data integrity
- Git-friendly format for backup and sync

#### CLI Commands
- `omniscient init` - Generate shell hooks
- `omniscient capture` - Capture command (internal)
- `omniscient search <query>` - Search command history
- `omniscient recent <n>` - Show recent commands
- `omniscient top <n>` - Show most used commands
- `omniscient category <name>` - Filter by category
- `omniscient stats` - Display statistics
- `omniscient export <file>` - Export to JSON
- `omniscient import <file>` - Import from JSON
- `omniscient config` - Show configuration

#### Categories
- `git` - Git version control commands
- `docker` - Docker and container commands
- `package` - Package managers (npm, cargo, pip, etc.)
- `file` - File operations (ls, cp, mv, etc.)
- `network` - Network utilities (curl, wget, ssh, etc.)
- `build` - Build tools (make, cmake, etc.)
- `database` - Database clients (psql, mysql, etc.)
- `kubernetes` - Kubernetes commands
- `cloud` - Cloud provider CLIs (aws, gcloud, az)
- `editor` - Text editors (vim, nano, emacs)
- `system` - System commands (sudo, systemctl, etc.)
- `vcs` - Version control (svn, hg)
- `other` - Uncategorized commands

### Features

#### Privacy & Security
- Automatic redaction of sensitive patterns (password, token, secret, api_key)
- Configurable redaction patterns
- Local-only storage (no telemetry)
- File permissions: 600 (owner read/write only)
- Enable/disable toggle for redaction

#### Performance
- Command capture: ~5ms (background, non-blocking)
- Search queries: < 500ms with 1,000+ commands
- Stats calculation: < 500ms
- Memory usage: < 50MB
- Binary size: 5.2MB
- SQLite indexes for fast queries

#### Configuration
- Default config auto-generation
- Tilde (~) expansion in paths
- Environment-aware configuration
- Directory auto-creation
- TOML-based configuration

### Testing
- 75 comprehensive unit tests
- 85% code coverage
- Integration tests for full workflow
- Performance benchmarks
- Export/import round-trip tests

### Quality
- Zero compiler warnings
- Zero clippy warnings
- Type-safe Rust implementation
- No unwrap() in production code
- Comprehensive error handling

### Documentation
- Complete README with examples
- Inline code documentation
- Configuration examples
- FAQ section
- Installation instructions

## [Unreleased]

### Planned Features
- Bash, Fish, PowerShell support
- Multi-line command handling
- Command execution with safety checks
- Web UI for history browsing
- AI-powered command suggestions
- Colorized terminal output
- User-defined tags
- Command templates

---

[1.0.0]: https://github.com/daneb/omniscient/releases/tag/v1.0.0
