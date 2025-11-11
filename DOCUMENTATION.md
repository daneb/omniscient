# Documentation Guide

Welcome! This guide helps you find the right documentation for your needs.

## 🎯 I Want To...

### Use Omniscient
→ Start with **[README.md](README.md)**

### Install Omniscient
→ See **[README: Installation](README.md#installation)** or use **[install.sh](install.sh)**

### Configure Omniscient
→ Check **[examples/config.toml](examples/config.toml)** for all options

### Contribute Code
→ Read **[CONTRIBUTING.md](CONTRIBUTING.md)**

### Understand the Architecture
→ See **[docs/planning/technical-design.md](docs/planning/technical-design.md)**

### Track Project Progress
→ View **[docs/development/PROJECT-SUMMARY.md](docs/development/PROJECT-SUMMARY.md)**

## 📚 Documentation Structure

```
omniscient/
│
├── 📖 User Documentation (Root)
│   ├── README.md              ⭐ Start here!
│   ├── CHANGELOG.md           📝 Version history
│   ├── CONTRIBUTING.md        🤝 How to contribute
│   ├── LICENSE                ⚖️  MIT License
│   └── RELEASE-NOTES.md       🎉 v1.0.0 release info
│
├── 📁 docs/                   📚 All documentation
│   ├── INDEX.md              🗂️  Complete navigation
│   ├── README.md             📋 Docs overview
│   │
│   ├── planning/             📐 Design documents
│   │   ├── specification.md
│   │   ├── technical-design.md
│   │   └── roadmap.md
│   │
│   └── development/          🔧 Dev progress
│       ├── PROJECT-SUMMARY.md
│       ├── PROGRESS.md
│       └── ...
│
└── 💡 examples/              📝 Config examples
    ├── config.toml
    └── zsh_hook.sh
```

## 🔗 Quick Navigation

### For Users
- **Quick Start**: [README.md](README.md) → Installation → Usage
- **All Commands**: [README: Usage](README.md#usage)
- **Configuration**: [examples/config.toml](examples/config.toml)
- **Help**: [README: FAQ](README.md#faq)

### For Developers
- **Getting Started**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Architecture**: [docs/planning/technical-design.md](docs/planning/technical-design.md)
- **Progress**: [docs/development/PROJECT-SUMMARY.md](docs/development/PROJECT-SUMMARY.md)
- **Full Index**: [docs/INDEX.md](docs/INDEX.md)

## 📖 Document Descriptions

### Root Directory (User-Facing)

| File | Purpose |
|------|---------|
| `README.md` | Main documentation, installation, usage |
| `CHANGELOG.md` | All version changes and features |
| `CONTRIBUTING.md` | Developer guidelines |
| `LICENSE` | MIT License text |
| `RELEASE-NOTES.md` | Detailed v1.0.0 information |
| `install.sh` | Automated installation script |

### docs/ (Comprehensive Documentation)

| Path | Purpose |
|------|---------|
| `docs/INDEX.md` | Complete documentation index |
| `docs/README.md` | Docs directory overview |
| `docs/planning/` | Original specs and design |
| `docs/development/` | Progress and summaries |

### examples/ (Configurations)

| File | Purpose |
|------|---------|
| `config.toml` | Example configuration with all options |
| `zsh_hook.sh` | Generated shell integration hook |

## 🎓 Learning Paths

### New User
1. Read [README.md](README.md)
2. Follow [Quick Start](README.md#quick-start)
3. Check [examples/config.toml](examples/config.toml)

### New Contributor
1. Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. Review [docs/planning/technical-design.md](docs/planning/technical-design.md)
3. Check [docs/development/PROJECT-SUMMARY.md](docs/development/PROJECT-SUMMARY.md)

### Project Maintainer
1. Review [docs/development/PROJECT-SUMMARY.md](docs/development/PROJECT-SUMMARY.md)
2. Check [docs/planning/roadmap.md](docs/planning/roadmap.md)
3. Update [CHANGELOG.md](CHANGELOG.md) for releases

## 🔍 Finding Information

### By Topic

- **Installation** → README.md or install.sh
- **Usage** → README.md or docs/INDEX.md
- **Configuration** → examples/config.toml
- **Features** → CHANGELOG.md or README.md
- **Contributing** → CONTRIBUTING.md
- **Architecture** → docs/planning/technical-design.md
- **Progress** → docs/development/
- **Planning** → docs/planning/

### By File Type

- **Markdown (*.md)** - All documentation
- **TOML (*.toml)** - Configuration files
- **Shell (*.sh)** - Scripts and hooks

## 💡 Pro Tips

- Use **[docs/INDEX.md](docs/INDEX.md)** for comprehensive navigation
- Check **[CHANGELOG.md](CHANGELOG.md)** for what's new
- Read **[examples/config.toml](examples/config.toml)** for configuration help
- Visit **[CONTRIBUTING.md](CONTRIBUTING.md)** before submitting PRs

## 📞 Need Help?

Can't find what you're looking for?
- Check [docs/INDEX.md](docs/INDEX.md) for complete navigation
- Read the [FAQ](README.md#faq)
- Open an [issue](https://github.com/yourusername/omniscient/issues)

---

**Version**: 1.0.0
**Last Updated**: 2025-11-11
