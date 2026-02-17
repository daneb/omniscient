# Publishing Guide

Complete guide for publishing Omniscient to crates.io and creating GitHub releases.

## Prerequisites

### 1. crates.io Account

```bash
# Create account at https://crates.io
# Get API token from https://crates.io/me

# Login to cargo
cargo login <your-api-token>
# Token is saved to ~/.cargo/credentials.toml
```

### 2. Verify Package Contents

```bash
# Preview what will be published
cargo package --list

# Build the package (creates target/package/)
cargo package

# Test the packaged version
cargo install --path target/package/omniscient-1.2.1
```

## Publishing to crates.io

### Step 1: Pre-Publish Checks

```bash
# Ensure all changes are committed
git status

# Run all tests
cargo test --verbose

# Check for warnings
cargo clippy -- -D warnings

# Build release
cargo build --release

# Verify version in Cargo.toml
grep "^version" Cargo.toml
```

### Step 2: Dry Run

```bash
# Test publishing without actually doing it
cargo publish --dry-run

# This will:
# - Build the package
# - Run tests
# - Verify metadata
# - Check for issues
```

### Step 3: Publish

```bash
# Actually publish to crates.io
cargo publish

# This will:
# - Upload to crates.io
# - Make it available within minutes
# - Be permanent (cannot unpublish after 72 hours)
```

### Step 4: Verify

```bash
# Wait ~2 minutes, then check
cargo search omniscient

# Or visit
open https://crates.io/crates/omniscient

# Test installation
cargo install omniscient --version 1.2.1
```

## Creating GitHub Releases

### Option 1: Using GitHub CLI (Recommended)

```bash
# Install gh CLI if needed
# macOS: brew install gh
# Or: https://cli.github.com/

# Authenticate
gh auth login

# Create release from existing tag
gh release create v1.2.1 \
  --title "v1.2.1: Security & Trust Infrastructure" \
  --notes-file - <<EOF
## Security & Trust Infrastructure

This release adds comprehensive security scanning and trust-building measures. No functional code changes.

### 🔒 Security Infrastructure

- GitHub Actions CI/CD (automated testing on Ubuntu & macOS)
- cargo-audit security scanning on every commit
- cargo-deny dependency verification (license & source checking)
- Code coverage tracking with codecov
- Snyk vulnerability monitoring workflow

### 📖 Documentation

- **SECURITY.md** - Comprehensive security policy and disclosure process
- **Enhanced CONTRIBUTING.md** - AI transparency and development process
- **Trust badges** - CI status, security audit results
- **Snyk setup guide** - Optional continuous security monitoring

### ✅ Trust Measures

- 91 automated tests on every commit
- Zero clippy warnings enforced
- Security audits automated
- Dependency license verification
- Zero network calls (verifiable with \`grep -r "http" src/\`)

### 🔍 Verification

\`\`\`bash
# Audit the code yourself
git clone https://github.com/daneb/omniscient
cd omniscient

# Run all tests
cargo test --verbose  # 91/91 passing

# Security audit
cargo install cargo-audit
cargo audit  # Zero vulnerabilities

# Verify no network calls
grep -r "http" src/ --include="*.rs" | grep -v "^[[:space:]]*//"
\`\`\`

### 📚 Documentation

- [SECURITY.md](https://github.com/daneb/omniscient/blob/master/SECURITY.md)
- [CONTRIBUTING.md](https://github.com/daneb/omniscient/blob/master/CONTRIBUTING.md)
- [ADR-002: Contextual Queries](https://github.com/daneb/omniscient/blob/master/docs/adr/ADR-002-contextual-queries.md)

### 🙏 Feedback

Questions or concerns? Please:
- Report security issues via SECURITY.md
- Open GitHub issues for bugs/features
- Join discussions for general questions

---

**Install**: \`cargo install omniscient\`
**Previous release**: [v1.2.0](https://github.com/daneb/omniscient/releases/tag/v1.2.0)
EOF
```

### Option 2: Using GitHub Web UI

1. Go to https://github.com/daneb/omniscient/releases
2. Click "Draft a new release"
3. Click "Choose a tag" → select `v1.2.1`
4. Title: `v1.2.1: Security & Trust Infrastructure`
5. Paste release notes (see template below)
6. Check "Set as the latest release"
7. Click "Publish release"

### Release Notes Template

```markdown
## Security & Trust Infrastructure

This release adds comprehensive security scanning and trust-building measures.

### 🔒 New Security Infrastructure

- GitHub Actions CI/CD (Ubuntu & macOS)
- cargo-audit security scanning
- cargo-deny dependency verification
- codecov integration
- Snyk monitoring workflow

### 📖 Documentation

- SECURITY.md with disclosure policy
- Enhanced CONTRIBUTING.md with AI transparency
- Trust badges in README
- Verification steps for privacy

### ✅ Trust Through Transparency

- 91 automated tests
- Zero clippy warnings
- Zero network calls (verifiable)
- All code auditable

### 🔍 Verify Yourself

\`\`\`bash
git clone https://github.com/daneb/omniscient
cargo test --verbose  # 91/91 passing
cargo audit  # Zero vulnerabilities
grep -r "http" src/  # No network calls
\`\`\`

**Install**: \`cargo install omniscient\`
```

## Post-Publishing Checklist

### After crates.io Publish

- [ ] Verify at https://crates.io/crates/omniscient
- [ ] Check that version 1.2.1 shows up
- [ ] Test installation: `cargo install omniscient`
- [ ] Verify documentation renders correctly
- [ ] Check download count starts incrementing

### After GitHub Release

- [ ] Verify release appears at /releases
- [ ] Check that it's marked as "Latest"
- [ ] Ensure release notes render correctly
- [ ] Verify tag is linked
- [ ] Check that assets are available (if any)

### Update Documentation

- [ ] Update README badges if needed
- [ ] Announce on social media
- [ ] Post to Hacker News (see docs/HACKERNEWS_ANNOUNCEMENT.md)
- [ ] Consider announcing on:
  - Reddit: r/rust, r/commandline
  - Twitter/X
  - Rust Users Forum
  - This Week in Rust newsletter

## Troubleshooting

### "crate already published"

Can't republish same version. Bump version and try again.

### "failed to verify package"

Check:
```bash
cargo package --list  # What will be published?
cargo package  # Build it
cargo test  # In target/package/
```

### "missing required fields"

Check Cargo.toml has:
- name, version, edition
- authors, description, license
- repository, homepage, documentation

### "file too large"

Check exclude list in Cargo.toml:
```toml
exclude = [
    "*.md",
    "examples/",
    "docs/",
    "install.sh",
]
```

## Version Strategy

- **Patch (1.2.x)**: Bug fixes, documentation, infrastructure
- **Minor (1.x.0)**: New features, non-breaking changes
- **Major (x.0.0)**: Breaking changes

## Automation (Future)

Consider automating with GitHub Actions:

```yaml
# .github/workflows/publish.yml
name: Publish to crates.io

on:
  push:
    tags:
      - 'v*.*.*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish --token ${{ secrets.CARGO_TOKEN }}
```

## Security Note

**Never commit your crates.io API token!**

- Token is in `~/.cargo/credentials.toml`
- For CI/CD, use GitHub secrets
- Rotate token if accidentally exposed

---

## Quick Reference

```bash
# Full release workflow
git tag -a v1.2.1 -m "Release notes"
git push origin v1.2.1
cargo publish
gh release create v1.2.1 --notes "..."

# Or step by step
cargo test
cargo clippy
cargo publish --dry-run
cargo publish
gh release create v1.2.1
```
