# Omniscient v1.0.0 - Release Notes

**Release Date**: November 11, 2025
**Status**: Production Ready ✅

## Overview

Omniscient v1.0.0 is the first stable release of our CLI command history tracker. This release includes all core functionality for capturing, searching, and managing your shell command history with privacy-first features and excellent performance.

## What's New

### Core Features

✅ **Command Capture & Storage**
- Automatic capture via Zsh shell hooks
- SQLite database with full-text search (FTS5)
- Zero shell impact (~5ms background capture)
- Duplicate detection with usage count tracking

✅ **Smart Categorization**
- 80+ recognized commands
- 13 categories (git, docker, package, file, network, etc.)
- Automatic classification based on command patterns

✅ **Search & Analytics**
- Full-text search with relevance ranking
- Recent commands display
- Top commands by usage frequency
- Category-based filtering
- Comprehensive statistics dashboard

✅ **Export & Import**
- JSON export with versioning
- Three merge strategies (Skip, UpdateUsage, PreserveHigher)
- Git-friendly format for backup and sync
- Round-trip data integrity

✅ **Privacy & Security**
- Automatic redaction of sensitive patterns
- Configurable redaction rules
- Local-only storage (no telemetry)
- File permissions: 600 (owner-only access)

## Installation

### From Source

```bash
git clone https://github.com/yourusername/omniscient.git
cd omniscient
cargo install --path .
```

### Quick Install Script

```bash
curl -sSL https://raw.githubusercontent.com/yourusername/omniscient/main/install.sh | bash
```

## Getting Started

1. **Install omniscient** (see above)

2. **Initialize shell integration**:
   ```bash
   omniscient init >> ~/.zshrc
   source ~/.zshrc
   ```

3. **Start using** - commands are now automatically tracked!

4. **Search your history**:
   ```bash
   omniscient search "git commit"
   omniscient recent 20
   omniscient top 10
   omniscient stats
   ```

## Performance Metrics

Tested with 1,000+ commands:
- **Capture**: ~5ms (background, non-blocking)
- **Search**: < 500ms
- **Stats**: < 500ms
- **Binary Size**: 5.2MB
- **Memory**: < 50MB

## Quality Metrics

- **Test Coverage**: 85% (75 tests)
- **Code Quality**: Zero compiler warnings
- **Linting**: All clippy checks passed
- **Documentation**: Complete API docs and examples

## File Manifest

This release includes:
- `omniscient` - Main binary (5.2MB)
- `README.md` - Comprehensive documentation
- `CHANGELOG.md` - Detailed change history
- `LICENSE` - MIT License
- `CONTRIBUTING.md` - Contribution guidelines
- `install.sh` - Installation script
- `examples/` - Example configurations and hooks

## Known Limitations

- **Shell Support**: Currently Zsh only (Bash, Fish coming in v1.1)
- **Multi-line Commands**: Not yet supported
- **Platforms**: Tested on macOS and Linux

## Upgrade Notes

This is the first stable release, so there's nothing to upgrade from. Future versions will provide migration guides.

## Migration from Other Tools

If you're coming from tools like `history`, `atuin`, or `mcfly`:
1. Export your existing history if possible
2. Install omniscient
3. Use `omniscient import` to bring in old data (may require conversion)

## Security Notes

- All data is stored locally in `~/.omniscient/`
- Database file permissions are set to 600 (owner-only)
- Sensitive patterns are automatically redacted
- No telemetry or external communication

## Support & Community

- **Issues**: https://github.com/yourusername/omniscient/issues
- **Discussions**: https://github.com/yourusername/omniscient/discussions
- **Documentation**: https://github.com/yourusername/omniscient#readme

## Credits

Built with ❤️ using Rust 🦀

Special thanks to all contributors and early testers!

## Next Steps

See our [roadmap](./README.md#roadmap) for planned features in v1.1 and beyond.

---

**Download**: [GitHub Releases](https://github.com/yourusername/omniscient/releases/tag/v1.0.0)
**Documentation**: [README](./README.md)
**License**: [MIT](./LICENSE)
