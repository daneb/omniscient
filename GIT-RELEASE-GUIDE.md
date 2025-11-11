# Git Release Guide - v1.0.0

Quick reference for creating and pushing the v1.0.0 release tag.

## 🚀 Quick Start

### Option 1: Use the Script (Easiest)
```bash
./release-tag.sh
```

### Option 2: Manual Commands
```bash
# 1. Stage all changes
git add .

# 2. Commit
git commit -m "Release v1.0.0 - Production ready CLI command history tracker"

# 3. Create tag
git tag -a v1.0.0 -m "Omniscient v1.0.0 - First stable release"

# 4. Push to remote
git push origin master
git push origin v1.0.0

# Or push both at once
git push --follow-tags
```

## 📝 Detailed Steps

### 1. Check Status
```bash
git status
git log --oneline -5
```

### 2. Stage Changes
```bash
# Stage all
git add .

# Or stage specific files
git add README.md CHANGELOG.md src/
```

### 3. Create Commit
```bash
git commit -m "Release v1.0.0 - Production ready CLI command history tracker

Complete implementation with:
- Command capture via Zsh
- Smart categorization (13 categories)
- Full-text search with FTS5
- Export/Import functionality
- Privacy-first redaction
- 75 tests, 85% coverage"
```

### 4. Create Annotated Tag
```bash
git tag -a v1.0.0 -m "Omniscient v1.0.0

First stable release.
See RELEASE-NOTES.md for details."
```

### 5. Push to Remote
```bash
# Push commit
git push origin master

# Push tag
git push origin v1.0.0

# Or both at once
git push --follow-tags
```

## 🔍 Verification

### Check Local Tags
```bash
git tag -l
git show v1.0.0
```

### Check Remote
```bash
git ls-remote --tags origin
```

### On GitHub
1. Go to your repository
2. Click "Releases" or "Tags"
3. Verify v1.0.0 appears

## 📦 Create GitHub Release

After pushing the tag:

1. Go to GitHub repository
2. Click "Releases" → "Draft a new release"
3. Choose tag: v1.0.0
4. Title: "Omniscient v1.0.0 - First Stable Release"
5. Copy content from `RELEASE-NOTES.md`
6. Attach binaries (optional)
7. Click "Publish release"

## 🔧 Troubleshooting

### Tag Already Exists Locally
```bash
# Delete local tag
git tag -d v1.0.0

# Recreate it
git tag -a v1.0.0 -m "Your message"
```

### Tag Already Exists on Remote
```bash
# Delete remote tag
git push origin :refs/tags/v1.0.0

# Push new tag
git push origin v1.0.0
```

### Force Push Tag
```bash
# If you need to update a tag
git tag -f -a v1.0.0 -m "Updated message"
git push -f origin v1.0.0
```

### Undo Last Commit (Before Push)
```bash
# Keep changes
git reset --soft HEAD^

# Discard changes
git reset --hard HEAD^
```

## 📋 Pre-Push Checklist

Before pushing, verify:
- [ ] All tests passing (`cargo test`)
- [ ] Build successful (`cargo build --release`)
- [ ] Version in Cargo.toml is 1.0.0
- [ ] CHANGELOG.md updated
- [ ] README.md current
- [ ] No uncommitted changes
- [ ] On correct branch (master)

## 🎯 Post-Push Steps

1. **Verify on GitHub**
   - Check Releases tab
   - Verify tag appears

2. **Create GitHub Release**
   - Use RELEASE-NOTES.md content
   - Add download instructions

3. **Announce** (optional)
   - Tweet/social media
   - Post to relevant communities
   - Update project website

4. **Cargo Publish** (when ready)
   ```bash
   cargo publish --dry-run
   cargo publish
   ```

## 📚 Git Tag Commands Reference

```bash
# List all tags
git tag -l

# Show tag details
git show v1.0.0

# Create lightweight tag
git tag v1.0.0

# Create annotated tag (recommended)
git tag -a v1.0.0 -m "Message"

# Tag specific commit
git tag -a v1.0.0 <commit-hash> -m "Message"

# Delete local tag
git tag -d v1.0.0

# Delete remote tag
git push origin :refs/tags/v1.0.0

# Push specific tag
git push origin v1.0.0

# Push all tags
git push --tags

# Push commits and tags
git push --follow-tags
```

## 🔗 Useful Links

- [Git Tagging Documentation](https://git-scm.com/book/en/v2/Git-Basics-Tagging)
- [GitHub Releases Guide](https://docs.github.com/en/repositories/releasing-projects-on-github)
- [Semantic Versioning](https://semver.org/)

---

**Your Project**: Omniscient v1.0.0
**Remote**: origin (GitHub)
**Branch**: master
