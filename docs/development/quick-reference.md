# Omniscient - Quick Reference

## 30-Second Overview
Cross-platform CLI command tracker that captures every command you run, categorizes them automatically, and makes them searchable. Built in Rust for speed and reliability. Survives machine migrations via Git-based sync.

## Core Value Proposition
**Never forget a command again.** Access your complete CLI history anytime, anywhere.

---

## Key Features (v1.0)

| Feature | Description | Status |
|---------|-------------|--------|
| **Auto-Capture** | Zsh hook captures every command | 📋 Planned |
| **Smart Categories** | Auto-categorize by type (git, docker, etc.) | 📋 Planned |
| **Fast Search** | Find commands in milliseconds | 📋 Planned |
| **Usage Tracking** | Rank by frequency | 📋 Planned |
| **Privacy** | Auto-redact sensitive data | 📋 Planned |
| **Export/Import** | JSON-based portability | 📋 Planned |
| **Git Sync** | Version control your commands | 📋 Planned |

---

## Architecture at a Glance

```
Zsh Hook → Capture → [Redact → Categorize → Store] → SQLite
                                                          ↓
User → Search CLI → Query Engine → Rank → Display
```

**Storage**: SQLite with FTS5 for fast search  
**Language**: Rust 2021  
**Distribution**: Single static binary

---

## Key Commands

```bash
# Setup
omniscient init                    # Generate shell hook

# Search
omniscient search "docker"         # Find commands
omniscient recent 20               # Last 20 commands
omniscient top 10                  # Most used
omniscient category git            # Filter by type

# Sync
omniscient export history.json     # Backup
omniscient import history.json     # Restore

# Info
omniscient stats                   # Usage statistics
```

---

## Data Model

```rust
CommandRecord {
    id: i64,
    command: String,              // "git commit -m 'fix'"
    timestamp: DateTime,          // When executed
    exit_code: i32,               // 0 = success
    duration_ms: i64,             // How long it took
    working_dir: String,          // Where it ran
    category: String,             // "git"
    usage_count: i32,             // How many times run
    last_used: DateTime,          // Most recent use
}
```

---

## File Structure

```
~/.omniscient/
├── history.db                    # SQLite database
└── config.toml                   # Configuration

/usr/local/bin/
└── omniscient                    # Single binary
```

---

## Configuration Example

```toml
[storage]
type = "sqlite"
path = "~/.omniscient/history.db"

[privacy]
redact_patterns = ["password", "token", "secret"]
enabled = true

[capture]
min_duration_ms = 0
max_history_size = 100000
```

---

## Development Phases

1. **Week 1**: Core Infrastructure + Capture
2. **Week 2**: Search & Retrieval + Export/Import
3. **Week 3**: Testing & Documentation
4. **Week 4**: Release Preparation

**Total**: 3-4 weeks to v1.0

---

## Technical Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 2021 |
| CLI | clap 4.4 |
| Database | rusqlite 0.30 (SQLite) |
| Serialization | serde 1.0 + serde_json |
| Date/Time | chrono 0.4 |
| Regex | regex 1.10 |
| Error Handling | thiserror 1.0 |
| Async | tokio 1.35 |

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Capture Overhead | < 10ms |
| Search (100k cmds) | < 100ms |
| Binary Size | < 10MB |
| Memory Usage | < 50MB |

---

## Security Considerations

- All data stored locally
- File permissions: 600 (owner only)
- Auto-redaction of sensitive patterns
- No telemetry or cloud sync
- User controls export/sync

---

## Future Enhancements (Post-v1.0)

### Deferred to v2.0+
- Bash, Fish, PowerShell support
- Multi-line command handling
- Command aliases expansion
- Team sharing features
- AI-powered suggestions
- Web UI
- Cloud sync options

---

## Getting Started

### Install
```bash
cargo install omniscient
```

### Setup
```bash
omniscient init >> ~/.zshrc
source ~/.zshrc
```

### Use
```bash
# Just use your shell normally
git status
docker ps

# Later, search
omniscient search "git"
omniscient top 5
```

---

## Project Files Reference

| File | Purpose |
|------|---------|
| SPECIFICATION.md | Complete functional spec |
| TECHNICAL_DESIGN.md | Architecture & implementation |
| ROADMAP.md | Development plan & timeline |
| README.md | User documentation |
| START_HERE.md | Development checklist |
| THIS_FILE.md | Quick reference |

---

## Success Criteria

- ✅ Captures commands with < 10ms overhead
- ✅ Searches 100k commands in < 100ms
- ✅ Works on Linux and macOS
- ✅ Single binary < 10MB
- ✅ Zero configuration required for basic use
- ✅ Survives machine migrations
- ✅ 80%+ test coverage
- ✅ Clear documentation

---

## Key Design Principles

1. **Lean**: Minimal dependencies, small binary
2. **Simple**: Easy to understand and use
3. **Performant**: Fast capture, fast search
4. **Human-First**: Built for real developer workflows
5. **Privacy**: Local-first, user-controlled

---

## Common Workflows

### Daily Use
```bash
# Automatic - just work normally
cd project
git status
cargo build
# ... etc ...

# Search when needed
omniscient search "cargo"
```

### Machine Migration
```bash
# Old machine
omniscient export ~/.omniscient-backup/history.json
cd ~/.omniscient-backup
git add history.json
git commit -m "Update history"
git push

# New machine
git clone <your-backup-repo> ~/.omniscient-backup
omniscient import ~/.omniscient-backup/history.json
```

### Find That Command
```bash
# "What was that docker command I used?"
omniscient search "docker volume"

# "Show me my most used git commands"
omniscient category git | omniscient top 10

# "What did I run yesterday?"
omniscient recent 50
```

---

## Contact & Contributing

- **Issues**: GitHub Issues
- **Contributions**: Pull Requests welcome
- **License**: MIT
- **Author**: Dane Balia

---

**Status**: 📋 Planning Complete - Ready to Build  
**Next**: Start Phase 1.2 - Storage Layer  
**Version**: 1.0.0-alpha

---

*"Never forget a command again." 🔍*