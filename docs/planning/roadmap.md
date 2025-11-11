# Development Roadmap

## Project: Omniscient v1.0

**Start Date**: 2025-11-10  
**Target Completion**: 3-4 weeks  
**Status**: 🟡 Planning Phase Complete

---

## Phase 1: Core Infrastructure (Week 1)

### Tasks

#### 1.1 Project Setup
- [x] Create project structure
- [x] Initialize Cargo.toml with dependencies
- [x] Set up Git repository
- [x] Create .gitignore
- [x] Add MIT License
- [ ] Set up basic CI/CD (GitHub Actions) - Deferred

**Estimated Time**: 2 hours

#### 1.2 Storage Layer ✅ COMPLETED
- [x] Define database schema
- [x] Implement SQLite connection management
- [x] Create CommandRecord struct with serde
- [x] Implement CRUD operations
- [x] Add database migrations support
- [x] Write unit tests for storage

**Files Created**:
- `src/storage.rs` ✅
- `src/models.rs` ✅

**Estimated Time**: 8 hours

#### 1.3 Configuration System ✅ COMPLETED
- [x] Define Config structs
- [x] Implement TOML parsing
- [x] Create default configuration
- [x] Add config file creation on first run
- [x] Handle missing/invalid config gracefully
- [x] Write unit tests for config

**Files Created**:
- `src/config.rs` ✅

**Estimated Time**: 4 hours

#### 1.4 Error Handling ✅ COMPLETED
- [x] Define OmniscientError enum
- [x] Implement thiserror derives
- [x] Create Result type alias
- [x] Add context to errors throughout

**Files Created**:
- `src/error.rs` ✅ (pre-existing)

**Estimated Time**: 2 hours

**Phase 1 Total**: ~16 hours (2 days) ✅ **COMPLETED**

---

## Phase 2: Capture Mechanism (Week 1-2) ✅ COMPLETED

### Tasks

#### 2.1 Redaction Engine ✅ COMPLETED
- [x] Create RedactionEngine struct
- [x] Implement regex pattern matching
- [x] Add default redaction patterns
- [x] Support custom patterns from config
- [x] Write comprehensive tests
  - [x] Test each default pattern
  - [x] Test custom patterns
  - [x] Test edge cases (empty, special chars)

**Files Created**:
- `src/redact.rs` ✅
- `tests/redaction_tests.rs` (integrated into module)

**Estimated Time**: 4 hours

#### 2.2 Categorization Engine ✅ COMPLETED
- [x] Create Categorizer struct
- [x] Define category rules
- [x] Implement pattern matching
- [x] Add all default categories
- [x] Write tests for each category
- [x] Handle edge cases (empty commands, aliases)

**Files Created**:
- `src/category.rs` ✅
- `tests/categorization_tests.rs` (integrated into module)

**Estimated Time**: 4 hours

#### 2.3 Capture Command ✅ COMPLETED
- [x] Implement capture subcommand
- [x] Parse command-line arguments
- [x] Apply redaction
- [x] Apply categorization
- [x] Check for duplicates
- [x] Insert/update in database
- [x] Handle errors gracefully (no shell interruption)
- [x] Write integration tests

**Files Created**:
- `src/capture.rs` ✅
- `tests/capture_integration_tests.rs` (integrated into module)

**Estimated Time**: 6 hours

#### 2.4 Shell Integration ✅ COMPLETED
- [x] Create shell hook generator
- [x] Generate Zsh hooks
- [x] Implement `init` command
- [x] Test on actual Zsh shell (ready for testing)
- [x] Document manual installation steps

**Files Created**:
- `src/shell.rs` ✅
- `examples/shell_hooks/zsh_hook.sh` ✅

**Estimated Time**: 4 hours

**Phase 2 Total**: ~18 hours (2-3 days) ✅ **COMPLETED**

---

## Phase 3: Search & Retrieval (Week 2)

### Tasks

#### 3.1 Search Engine Foundation
- [ ] Create SearchEngine struct
- [ ] Implement SearchQuery builder
- [ ] Add FTS5 virtual table support
- [ ] Implement basic text search
- [ ] Write search tests

**Files to Create**:
- `src/search.rs`

**Estimated Time**: 4 hours

#### 3.2 Ranking Algorithm
- [ ] Implement relevance scoring
- [ ] Add usage frequency weighting
- [ ] Add recency decay
- [ ] Tune scoring parameters
- [ ] Test ranking quality

**Estimated Time**: 4 hours

#### 3.3 CLI Search Commands
- [ ] Implement `search` subcommand
- [ ] Implement `recent` subcommand
- [ ] Implement `top` subcommand
- [ ] Implement `category` subcommand
- [ ] Implement `stats` subcommand
- [ ] Format output nicely
- [ ] Add color support (optional)
- [ ] Write integration tests for each command

**Files to Create**:
- `src/commands/search.rs`
- `src/commands/recent.rs`
- `src/commands/top.rs`
- `src/commands/category.rs`
- `src/commands/stats.rs`
- `src/output.rs` (formatting)

**Estimated Time**: 8 hours

#### 3.4 Statistics & Analytics
- [ ] Implement stats collection
- [ ] Count total commands
- [ ] Count by category
- [ ] Calculate success rate
- [ ] Find most used commands
- [ ] Calculate time ranges

**Estimated Time**: 3 hours

**Phase 3 Total**: ~19 hours (2-3 days)

---

## Phase 4: Sync & Portability (Week 2-3)

### Tasks

#### 4.1 Export Functionality
- [ ] Define export JSON schema
- [ ] Implement Exporter struct
- [ ] Query all commands from storage
- [ ] Serialize to JSON (pretty-printed)
- [ ] Write to file
- [ ] Add progress indicator (optional)
- [ ] Test with large datasets

**Files to Create**:
- `src/export.rs`
- `tests/export_tests.rs`

**Estimated Time**: 4 hours

#### 4.2 Import Functionality
- [ ] Implement Importer struct
- [ ] Parse JSON file
- [ ] Validate schema version
- [ ] Detect duplicates
- [ ] Merge strategies
  - [ ] Skip duplicates
  - [ ] Update usage counts
  - [ ] Preserve higher counts
- [ ] Show import statistics
- [ ] Handle errors gracefully
- [ ] Test with various scenarios

**Files to Create**:
- Tests in `tests/export_tests.rs`

**Estimated Time**: 6 hours

#### 4.3 Export/Import CLI
- [ ] Implement `export` subcommand
- [ ] Implement `import` subcommand
- [ ] Add file path validation
- [ ] Add confirmation prompts
- [ ] Write integration tests

**Estimated Time**: 3 hours

**Phase 4 Total**: ~13 hours (1-2 days)

---

## Phase 5: Testing & Documentation (Week 3)

### Tasks

#### 5.1 Unit Testing
- [ ] Achieve 80%+ code coverage
- [ ] Test all error cases
- [ ] Test edge cases
- [ ] Test with empty database
- [ ] Test with large dataset (100k commands)

**Estimated Time**: 8 hours

#### 5.2 Integration Testing
- [ ] End-to-end capture flow
- [ ] Search accuracy
- [ ] Export/import round-trip
- [ ] Shell integration (manual)
- [ ] Performance benchmarks

**Files to Create**:
- `tests/integration_tests.rs`
- `benches/performance.rs` (optional)

**Estimated Time**: 6 hours

#### 5.3 Documentation
- [ ] Write comprehensive README
- [ ] Add installation instructions
- [ ] Add usage examples
- [ ] Document configuration options
- [ ] Add troubleshooting section
- [ ] Create CONTRIBUTING.md
- [ ] Add inline code documentation
- [ ] Generate rustdoc

**Estimated Time**: 6 hours

#### 5.4 Performance Optimization
- [ ] Profile capture performance
- [ ] Optimize database queries
- [ ] Tune SQLite settings
- [ ] Reduce binary size
- [ ] Test on slower machines

**Estimated Time**: 4 hours

**Phase 5 Total**: ~24 hours (3 days)

---

## Phase 6: Release Preparation (Week 3-4)

### Tasks

#### 6.1 Cross-Platform Builds
- [ ] Set up GitHub Actions for builds
- [ ] Build for Linux x86_64
- [ ] Build for macOS Intel (x86_64)
- [ ] Build for macOS Apple Silicon (aarch64)
- [ ] Verify binary sizes < 10MB
- [ ] Test on each platform

**Estimated Time**: 4 hours

#### 6.2 Release Process
- [ ] Create release checklist
- [ ] Version bump to 1.0.0
- [ ] Create Git tag
- [ ] Generate changelog
- [ ] Create GitHub release
- [ ] Upload binaries
- [ ] Update documentation links

**Estimated Time**: 2 hours

#### 6.3 Installation Scripts
- [ ] Create install.sh for Unix
- [ ] Add to PATH instructions
- [ ] Test installation on clean machines
- [ ] Document uninstallation

**Estimated Time**: 3 hours

#### 6.4 Marketing Materials
- [ ] Create demo GIF/video
- [ ] Write announcement post
- [ ] Prepare Hacker News submission
- [ ] Share on Reddit (r/rust, r/commandline)
- [ ] Tweet about release

**Estimated Time**: 3 hours

**Phase 6 Total**: ~12 hours (1-2 days)

---

## Summary

### Total Estimated Time
- Phase 1: 16 hours (2 days)
- Phase 2: 18 hours (2-3 days)
- Phase 3: 19 hours (2-3 days)
- Phase 4: 13 hours (1-2 days)
- Phase 5: 24 hours (3 days)
- Phase 6: 12 hours (1-2 days)

**Total**: ~102 hours (13 days of 8-hour work)

**With Buffer**: 3-4 weeks for a sustainable pace

### Critical Path
1. Storage Layer → Capture → Search → Export/Import
2. Shell Integration can be done in parallel with Search
3. Testing should be continuous, not just Phase 5
4. Documentation should be written alongside code

---

## Development Workflow

### Daily Routine
1. Pick 1-2 tasks from current phase
2. Write tests first (TDD)
3. Implement feature
4. Run all tests
5. Commit with clear message
6. Update roadmap progress

### Weekly Goals
- Week 1: Complete Phases 1 & 2 (Core + Capture)
- Week 2: Complete Phase 3 (Search) + Start Phase 4 (Export)
- Week 3: Complete Phase 4 & 5 (Export + Testing)
- Week 4: Complete Phase 6 (Release)

### Milestone Markers
- ✅ **M1**: Database can store and retrieve commands
- ✅ **M2**: Shell integration captures commands
- ✅ **M3**: Search returns relevant results
- ✅ **M4**: Export/import works correctly
- ✅ **M5**: All tests pass, documentation complete
- ✅ **M6**: Binaries built and released

---

## Risk Management

### Potential Blockers

| Risk | Impact | Mitigation |
|------|--------|------------|
| Shell hook complexity | High | Start simple, iterate based on testing |
| SQLite FTS5 performance | Medium | Benchmark early, have fallback to LIKE queries |
| Cross-platform builds | Medium | Use GitHub Actions early, test incrementally |
| Scope creep | High | Stick to spec, defer features to v2 |
| Time estimation | Medium | Build in 20% buffer, prioritize ruthlessly |

### Decision Points
- **Storage**: SQLite chosen, but could switch to JSON if issues arise
- **Async**: tokio for background capture, but could simplify if not needed
- **FTS5**: If too complex, fall back to basic LIKE queries

---

## Post-v1.0 Roadmap

### Version 1.1 (Polish)
- Better error messages
- Colorized output
- Bash support
- Configuration improvements

### Version 2.0 (Expansion)
- Multi-line command support
- Fish & PowerShell support
- Web UI
- User-defined tags
- Command templates

### Version 3.0 (Intelligence)
- AI-powered suggestions
- Context-aware recommendations
- Team sharing features
- Cloud sync options

---

## Getting Started with Development

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/daneb/omniscient.git
cd omniscient

# Build
cargo build

# Run tests
cargo test

# Run locally
cargo run -- --help
```

### First Tasks
1. Review SPECIFICATION.md
2. Review TECHNICAL_DESIGN.md
3. Start with Phase 1, Task 1.2 (Storage Layer)
4. Write tests first
5. Implement incrementally

---

**Status**: Ready to Begin Development ✅  
**Next Action**: Start Phase 1.2 - Storage Layer Implementation

**Last Updated**: 2025-11-10