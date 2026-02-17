# Snyk Setup Instructions

Snyk provides continuous security monitoring for vulnerabilities and license compliance.

## Setup Steps

### 1. Create Snyk Account

1. Go to https://snyk.io/
2. Sign up with your GitHub account (free for open source)
3. Verify your email

### 2. Connect Repository

1. In Snyk dashboard, click "Add project"
2. Select GitHub
3. Choose `omniscient` repository
4. Click "Add selected repositories"

### 3. Get Snyk Token

1. Go to Account Settings (https://app.snyk.io/account)
2. Copy your API token
3. Go to GitHub repository Settings → Secrets → Actions
4. Create new secret: `SNYK_TOKEN` with your API token

### 4. Enable GitHub Integration

In Snyk project settings:
- ✅ Enable "Automatic fix PRs"
- ✅ Enable "Automatic dependency upgrade PRs"
- ✅ Set "Test frequency" to Weekly
- ✅ Enable "PR checks" (blocks PRs with high vulnerabilities)

### 5. Add Badge to README

Once setup, add this to your README.md:

```markdown
[![Snyk Vulnerabilities](https://snyk.io/test/github/daneb/omniscient/badge.svg)](https://snyk.io/test/github/daneb/omniscient)
```

## What Snyk Monitors

- **Rust Dependencies**: All crates in Cargo.toml
- **Transitive Dependencies**: Deep dependency tree
- **License Compliance**: Ensures all dependencies use approved licenses
- **Security Advisories**: Multiple vulnerability databases

## Benefits

1. **Automated Fix PRs**: Snyk creates PRs to update vulnerable dependencies
2. **Continuous Monitoring**: Weekly scans even when you're not committing
3. **PR Checks**: Blocks merging PRs that introduce vulnerabilities
4. **Detailed Reports**: Shows vulnerability details and remediation steps
5. **Trust Badge**: Industry-recognized security badge for README

## Cost

Free for open source projects!

## Comparison with cargo-audit

| Feature | cargo-audit | Snyk |
|---------|-------------|------|
| Rust vulnerability DB | ✅ RustSec | ✅ RustSec + others |
| GitHub integration | ❌ | ✅ Automated PRs |
| Continuous monitoring | ❌ | ✅ Weekly scans |
| License checking | ❌ (use cargo-deny) | ✅ |
| Container scanning | ❌ | ✅ |
| Detailed reports | Basic | ✅ Comprehensive |
| Trust badge | ❌ | ✅ |
| Cost | Free | Free (OSS) |

**Recommendation**: Use both! cargo-audit in CI for fast checks, Snyk for continuous monitoring.

## After Setup

Your security setup will be:

1. **cargo-audit**: Fast check on every commit (CI)
2. **cargo-deny**: License and source verification (CI)
3. **Snyk**: Continuous monitoring + automated PRs (weekly)

This multi-layered approach gives users maximum confidence.
