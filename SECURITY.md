# Security Policy

## Our Commitment to Security

Omniscient is a command history tracker that stores your shell commands locally. We take security seriously because this tool has access to sensitive information.

### Core Security Principles

1. **Privacy First**: All data stored locally in `~/.omniscient/`. No network calls. No telemetry.
2. **Transparent Code**: Open source with comprehensive tests and ADRs documenting all decisions.
3. **Automated Security**: Continuous security audits via GitHub Actions.
4. **Responsible Development**: AI-assisted development with human review and validation.

## What We Protect

- **Your Command History**: Stored in a local SQLite database
- **Your Privacy**: Automatic redaction of passwords, tokens, and secrets
- **Your Data**: No data leaves your machine unless you explicitly export it

## Security Features

### Built-in Privacy Protection

```rust
// Automatic redaction of sensitive patterns (src/redact.rs)
- Passwords: "password", "passwd", "pwd="
- API Keys: "api_key", "apikey", "api-key"
- Tokens: "token", "auth", "bearer"
- Secrets: "secret", "private_key"
```

When a command matches these patterns, it is **not stored** (not even redacted - completely dropped).

### Data Storage Security

- **Local Only**: All data in `~/.omniscient/` directory
- **No Network Calls**: Zero network activity (verified in code)
- **No Telemetry**: No analytics, no tracking, no phone home
- **User Control**: Export/import allows you to manage your data

### Code Quality Assurance

- ✅ **91 Automated Tests**: Comprehensive test coverage
- ✅ **Zero Clippy Warnings**: Strict linting enforced
- ✅ **Security Audits**: Automated `cargo-audit` checks
- ✅ **Dependency Scanning**: `cargo-deny` for license/security
- ✅ **CI/CD Pipeline**: Every commit tested on Ubuntu & macOS

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.2.x   | :white_check_mark: |
| 1.1.x   | :white_check_mark: |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take all security reports seriously.

### How to Report

**For security vulnerabilities, please DO NOT open a public issue.**

Instead, please report security issues via:

1. **Email**: [Your email or security@yourdomain.com]
2. **GitHub Security Advisory**: Use the "Security" tab → "Report a vulnerability"

### What to Include

Please include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)
- Your contact information

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 24-48 hours
  - High: 1 week
  - Medium: 2 weeks
  - Low: Next release cycle

### Disclosure Policy

- We will work with you to understand and verify the issue
- We will develop a fix and coordinate disclosure timing
- We will credit you in the security advisory (unless you prefer anonymity)
- We will publish a security advisory once the fix is released

## Security Best Practices for Users

### Installation

```bash
# Verify the installation source
cargo install omniscient --git https://github.com/daneb/omniscient --tag v1.2.0

# Or build from source (recommended for maximum trust)
git clone https://github.com/daneb/omniscient
cd omniscient
git checkout v1.2.0
cargo build --release
```

### Data Management

```bash
# Backup your data
cp -r ~/.omniscient ~/.omniscient.backup

# Review what's being stored
omniscient recent 20

# Export for version control (private repo)
omniscient export ~/omniscient-backup.json

# Clear sensitive commands if needed
rm ~/.omniscient/history.db
```

### Custom Redaction

Add your own patterns to `~/.omniscient/config.toml`:

```toml
[privacy]
enabled = true
redact_patterns = [
    "password",
    "token",
    "my-secret-pattern",
]
```

## Security Audit Results

### Latest Audit: 2026-02-17

- ✅ `cargo audit`: No known security vulnerabilities
- ✅ `cargo deny`: All licenses approved
- ✅ `clippy`: Zero warnings
- ✅ Test Suite: 91/91 tests passing

### Dependency Security

All dependencies are from crates.io and vetted:
- `rusqlite`: Battle-tested SQLite wrapper
- `chrono`: Standard datetime library
- `clap`: Industry-standard CLI parser
- `serde`: De-facto Rust serialization
- All dependencies use permissive licenses (MIT/Apache-2.0)

## AI-Assisted Development & Trust

### Transparency

This project is developed with AI assistance (Claude). Here's how we ensure quality:

1. **Human Review**: All AI-generated code is reviewed and tested
2. **Comprehensive Testing**: 91 tests validate all functionality
3. **Architecture Decisions**: All major decisions documented in ADRs
4. **Open Development**: Full git history shows incremental development
5. **No Secrets**: AI doesn't have access to your data (runs locally)

### What You Can Verify

```bash
# Clone and audit the code yourself
git clone https://github.com/daneb/omniscient
cd omniscient

# Read the architecture decision records
ls docs/adr/

# Review the test coverage
cargo test --verbose

# Check for security issues
cargo install cargo-audit
cargo audit

# Verify no network calls (grep the source)
grep -r "http://" src/
grep -r "https://" src/
# Should return nothing (except comments/tests)
```

### Why Open Source Matters

- **Transparency**: You can read every line of code
- **Auditability**: Security researchers can review the codebase
- **Community**: Issues are tracked publicly
- **Control**: You can fork and modify as needed

## Continuous Security

We are committed to:

- ✅ Regular dependency updates
- ✅ Prompt security patch releases
- ✅ Transparent communication about issues
- ✅ Community engagement on security topics

## Questions?

For security questions (non-vulnerabilities):
- Open a GitHub Discussion
- Tag issues with `security` label

For urgent security matters, use the reporting process above.

---

**Last Updated**: 2026-02-17
**Current Version**: 1.2.0
