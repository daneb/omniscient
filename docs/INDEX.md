# Omniscient Documentation Index

Welcome to the Omniscient documentation! This index helps you navigate all available documentation.

## 📚 For Users

### Getting Started
- **[README](../README.md)** - Main project overview, installation, and quick start guide
- **[CHANGELOG](../CHANGELOG.md)** - Version history and release notes
- **[RELEASE-NOTES](../RELEASE-NOTES.md)** - Detailed v1.0.0 release information

### Configuration & Examples
- **[Example Config](../examples/config.toml)** - Sample configuration with all options
- **[Zsh Hook Example](../examples/zsh_hook.sh)** - Generated shell integration hook

### Contributing
- **[CONTRIBUTING](../CONTRIBUTING.md)** - How to contribute to the project
- **[LICENSE](../LICENSE)** - MIT License terms

## 🔧 For Developers

### Planning Documents
Located in [`docs/planning/`](planning/):
- **[Specification](planning/specification.md)** - Original project specification and requirements
- **[Technical Design](planning/technical-design.md)** - Architecture and design decisions
- **[Roadmap](planning/roadmap.md)** - Development phases and timeline
- **[Multi-Shell Support](planning/multi-shell-support.md)** - Bash, Fish, and PowerShell implementation plan

### Development Progress
Located in [`docs/development/`](development/):
- **[Project Summary](development/PROJECT-SUMMARY.md)** - Complete project overview and statistics
- **[Progress Tracking](development/PROGRESS.md)** - Phase-by-phase development progress
- **[Phase 2 Summary](development/PHASE2-SUMMARY-PROGRESS.md)** - Detailed Phase 2 completion report

### Quick References
Located in [`docs/development/`](development/):
- **[Quick Reference](development/quick-reference.md)** - Command and feature quick reference
- **[Review Checklist](development/review-checklist.md)** - Code review guidelines
- **[Start Here](development/START-HERE.md)** - Developer onboarding guide
- **[Original Index](development/Index.md)** - Original project index

## 📖 Documentation by Topic

### Installation
1. [Installation Instructions](../README.md#installation) - From source or Cargo
2. [Installation Script](../install.sh) - Automated installer for Unix systems

### Usage
1. [Quick Start](../README.md#quick-start) - Get up and running in 2 minutes
2. [Basic Commands](../README.md#usage) - Common command examples
3. [Configuration](../README.md#configuration) - Customizing Omniscient
4. [Example Config](../examples/config.toml) - All configuration options

### Features
1. [How It Works](../README.md#how-it-works) - Architecture overview
2. [Privacy & Security](../README.md#security) - Data handling and privacy
3. [Performance](../README.md#performance) - Benchmarks and metrics
4. [Categories](../CHANGELOG.md#categories) - Supported command categories

### Development
1. [Architecture](planning/technical-design.md) - System design
2. [Module Structure](development/PROJECT-SUMMARY.md#architecture) - Code organization
3. [Testing](../CONTRIBUTING.md#testing) - Test guidelines
4. [Code Quality](../CONTRIBUTING.md#code-quality-standards) - Standards and linting

## 🎯 Common Tasks

### For Users
- **Install Omniscient** → [README: Installation](../README.md#installation)
- **Set up shell integration** → [README: Setup](../README.md#setup-zsh)
- **Search commands** → [README: Basic Commands](../README.md#basic-commands)
- **Export history** → [README: Export & Sync](../README.md#export--sync)
- **Configure privacy** → [README: Privacy & Redaction](../README.md#privacy--redaction)

### For Contributors
- **Set up development** → [CONTRIBUTING: Development Setup](../CONTRIBUTING.md#development-setup)
- **Run tests** → [CONTRIBUTING: Running Tests](../CONTRIBUTING.md#running-tests)
- **Add features** → [CONTRIBUTING: Adding Features](../CONTRIBUTING.md#adding-features)
- **Submit PR** → [CONTRIBUTING: Submit Pull Request](../CONTRIBUTING.md#submit-pull-request)

### For Maintainers
- **Review releases** → [Release Notes](../RELEASE-NOTES.md)
- **Check progress** → [Project Summary](development/PROJECT-SUMMARY.md)
- **Plan features** → [Roadmap](planning/roadmap.md)
- **Multi-shell support** → [Multi-Shell Support Spec](planning/multi-shell-support.md)

## 📁 File Organization

```
omniscient/
├── README.md                    # Main user documentation
├── CHANGELOG.md                 # Version history
├── CONTRIBUTING.md              # Contribution guidelines
├── LICENSE                      # MIT License
├── RELEASE-NOTES.md            # Release information
├── install.sh                   # Installation script
│
├── docs/                        # Documentation
│   ├── INDEX.md                # This file
│   ├── planning/               # Planning documents
│   │   ├── specification.md
│   │   ├── technical-design.md
│   │   ├── roadmap.md
│   │   └── multi-shell-support.md
│   └── development/            # Development docs
│       ├── PROJECT-SUMMARY.md
│       ├── PROGRESS.md
│       └── ...
│
├── examples/                    # Example configurations
│   ├── config.toml
│   └── zsh_hook.sh
│
└── src/                        # Source code
    ├── main.rs
    ├── lib.rs
    └── ...
```

## 🔍 Quick Links

### Essential Reading
1. [README](../README.md) - Start here!
2. [Installation](../README.md#installation)
3. [Quick Start](../README.md#quick-start)
4. [Contributing](../CONTRIBUTING.md)

### Deep Dives
1. [Project Summary](development/PROJECT-SUMMARY.md) - Complete overview
2. [Technical Design](planning/technical-design.md) - Architecture details
3. [Specification](planning/specification.md) - Requirements and features

### Release Information
1. [v1.0.0 Release Notes](../RELEASE-NOTES.md)
2. [Changelog](../CHANGELOG.md)
3. [Roadmap](planning/roadmap.md)

## 💡 Need Help?

- **Users**: Start with the [README](../README.md) and [FAQ](../README.md#faq)
- **Developers**: Check [CONTRIBUTING](../CONTRIBUTING.md)
- **Issues**: Visit [GitHub Issues](https://github.com/yourusername/omniscient/issues)
- **Discussions**: Join [GitHub Discussions](https://github.com/yourusername/omniscient/discussions)

## 📝 Document Conventions

- **User-facing docs** are in the root directory (README, CHANGELOG, etc.)
- **Planning docs** are in `docs/planning/`
- **Development docs** are in `docs/development/`
- **Examples** are in `examples/`
- All Markdown files use GitHub-flavored Markdown

---

**Last Updated**: 2025-11-12
**Version**: 1.0.2
