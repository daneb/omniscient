# Omniscient 🔍

> Never forget a command again. Your complete CLI history, categorized and searchable.

**Omniscient** is a cross-platform command history tracker that captures every command you run, categorizes them intelligently, and makes them instantly searchable. Survive machine migrations, access your command library anywhere, and boost your CLI productivity.

📚 **[Complete Documentation](DOCUMENTATION.md)** | 📖 **[Full Index](docs/INDEX.md)** | 🤝 **[Contributing](CONTRIBUTING.md)**

## Features

- 🚀 **Zero Overhead**: < 10ms command capture, won't slow you down
- 📊 **Smart Categorization**: Automatic categorization by command type
- 🔍 **Fast Search**: Find any command in milliseconds, even with 100k+ history
- 🔒 **Privacy First**: Automatic redaction of sensitive data
- 💾 **Portable**: Export/import your history, sync via Git
- 📈 **Usage Analytics**: Track your most-used commands
- 🎯 **Simple**: Single binary, minimal configuration

## Quick Start

### Installation

#### From Source (Recommended for v1.0.0)

```bash
# Clone the repository
git clone https://github.com/daneb/omniscient.git
cd omniscient

# Build and install
cargo install --path .

# Verify installation
omniscient --version
```

#### Via Cargo 

```bash
cargo install omniscient
```

### Setup (Zsh)

```bash
# Initialize shell integration
omniscient init

# Add the output to your ~/.zshrc
omniscient init >> ~/.zshrc

# Reload your shell
source ~/.zshrc
```

That's it! Omniscient is now tracking your commands.

## Usage

### Basic Commands

```bash
# Search your command history
omniscient search "git commit"

# Show recent commands
omniscient recent 20

# Most frequently used commands
omniscient top 10

# Filter by category
omniscient category git

# View statistics
omniscient stats
```

### Export & Sync

```bash
# Export your history
omniscient export history.json

# Import on a new machine
omniscient import history.json

# Sync via Git (recommended workflow)
omniscient export ~/.omniscient-backup/history.json
cd ~/.omniscient-backup
git add history.json
git commit -m "Update command history"
git push
```

### Privacy & Redaction

Omniscient automatically redacts sensitive patterns. Configure in `~/.omniscient/config.toml`:

```toml
[privacy]
redact_patterns = ["password", "token", "secret", "api_key"]
enabled = true
```

## How It Works

Omniscient uses Zsh hooks to capture commands:
1. **preexec**: Starts a timer when you press Enter
2. **Command executes**: Your shell runs the command normally
3. **precmd**: Captures command, exit code, and duration
4. **Storage**: Saves to local SQLite database with categorization

Zero impact on your workflow, all happens in the background.

## Example Output

```
$ omniscient search "docker"

[2025-11-10 14:32:45] [✓] docker ps -a                          (git/123)
[2025-11-10 12:15:30] [✗] docker build -t myapp .               (docker/45)
[2025-11-09 16:45:12] [✓] docker-compose up -d                  (docker/89)
[2025-11-09 10:22:01] [✓] docker logs -f container_name         (docker/12)

$ omniscient top 5

1. git status                    (1,234 uses)
2. ls -la                        (892 uses)
3. cd ..                         (654 uses)
4. git commit -m                 (432 uses)
5. cargo build                   (301 uses)
```

## Configuration

Configuration file: `~/.omniscient/config.toml`

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

## Project Structure

```
~/.omniscient/
├── history.db      # SQLite database with your commands
└── config.toml     # Configuration file
```

## Performance

**Tested with 1,000+ commands:**
- Command capture: ~5ms (background, non-blocking)
- Search queries: < 500ms
- Stats calculation: < 500ms
- Memory usage: < 50MB
- Binary size: 5.2MB

## Security

- All data stored locally (`~/.omniscient/`)
- File permissions: 600 (owner read/write only)
- Automatic redaction of sensitive patterns
- No telemetry, no cloud sync (unless you choose Git)

## Roadmap

### Version 1.0 (Current)
- ✅ Zsh integration
- ✅ SQLite storage
- ✅ Command categorization
- ✅ Search and retrieval
- ✅ Export/import

### Future Versions
- Bash, Fish, PowerShell support
- Multi-line command handling
- Command execution with safety checks
- Web UI for history browsing
- AI-powered command suggestions

See [SPECIFICATION.md](SPECIFICATION.md) for detailed feature planning.

## Contributing

Contributions welcome! Please read our contributing guidelines first.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Development

```bash
# Clone the repository
git clone https://github.com/daneb/omniscient.git
cd omniscient

# Build
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

## Uninstallation

If you need to uninstall Omniscient:

### Automated Uninstall (Recommended)

```bash
# Download and run the uninstall script
curl -sSL https://raw.githubusercontent.com/daneb/omniscient/master/uninstall.sh | bash

# Or if you have the repository cloned
./uninstall.sh
```

The script will:
1. Remove shell hooks from your `~/.zshrc`
2. Remove the binary from `~/.cargo/bin/omniscient`
3. Optionally delete your command history data
4. Create backups before deletion

### Manual Uninstall

```bash
# 1. Remove shell hooks
# Edit ~/.zshrc and remove the "# Omniscient" section
vim ~/.zshrc

# 2. Remove the binary
cargo uninstall omniscient
# or manually
rm ~/.cargo/bin/omniscient

# 3. (Optional) Remove data directory
rm -rf ~/.omniscient

# 4. Reload shell
source ~/.zshrc
```

**Note**: The uninstaller creates backups before deletion. Your data will be backed up to `~/omniscient_backup_*` before removal.

## License

MIT License - see [LICENSE](LICENSE) for details.

## FAQ

**Q: Does this slow down my shell?**  
A: No. Capture happens asynchronously after command execution with < 10ms overhead.

**Q: What about sensitive data in commands?**  
A: Automatic redaction of common patterns (passwords, tokens, etc.). Configurable.

**Q: Can I use this across multiple machines?**  
A: Yes! Export your history to JSON and sync via Git or any file sync tool.

**Q: Does it capture command output?**  
A: No, only the command itself and metadata (exit code, duration, etc.). Keeps storage lean.

**Q: What if I delete my machine?**  
A: Export regularly and keep the JSON in version control. Import on your new machine.

**Q: Is my data sent anywhere?**  
A: Never. Everything is local unless you explicitly export and sync via Git.

## Acknowledgments

Built with Rust 🦀 for performance and reliability.

Inspired by the need to never lose a useful command again.

---

## Project Status

**Version**: 1.0.0
**Status**: ✅ Production Ready
**Maintained**: Yes
**Test Coverage**: 85% (75 tests passing)
**Code Quality**: Zero warnings, all clippy lints passed

### What's Working
- ✅ Command capture with automatic categorization
- ✅ Full-text search with FTS5
- ✅ Export/Import with multiple merge strategies
- ✅ Statistics and analytics
- ✅ Privacy-first redaction
- ✅ Zsh shell integration
- ✅ Comprehensive test suite

Star ⭐ this repo if you find it useful!