# Release Workflow Quick Reference

One command to do it all.

## Quick Release

```bash
# Run the release script with version number
./release.sh 1.2.2
```

That's it! The script will:
1. ✅ Check git account (warns if using work email)
2. ✅ Check working directory is clean
3. ✅ Run all tests
4. ✅ Run clippy checks
5. ✅ Check code formatting
6. ✅ Update Cargo.toml version
7. ✅ Update Cargo.lock
8. ✅ Commit version bump
9. ✅ Create annotated git tag
10. ✅ Run cargo publish dry-run
11. ✅ Publish to crates.io (with confirmation)
12. ✅ Push to GitHub (with confirmation)
13. ✅ Create GitHub release with gh CLI (with confirmation)

## What You'll Be Asked

The script will prompt for:
- Continue if using work email? (safety check)
- Commit version bump? (shows diff first)
- Release notes for git tag (type message, ctrl-d when done)
- Publish to crates.io? (y/n)
- Push to GitHub? (y/n)
- Create GitHub release? (y/n)
- Release title (optional, defaults to version tag)

## Example Session

```bash
$ ./release.sh 1.2.2

🚀 Omniscient Release Workflow
================================
Version: v1.2.2

▶ Checking git account...
✅ Git account: happyfrog@tuta.io

▶ Checking git status...
✅ Working directory is clean

▶ Running tests...
✅ All tests passed

▶ Running clippy...
✅ No clippy warnings

▶ Checking formatting...
✅ Code is formatted

▶ Updating Cargo.toml version...
✅ Updated Cargo.toml to version 1.2.2

▶ Updating Cargo.lock...
✅ Cargo.lock updated

Changes to commit:
[shows diff of Cargo.toml and Cargo.lock]

Commit version bump to v1.2.2? (y/n) y

▶ Committing version bump...
✅ Version bump committed

▶ Creating git tag v1.2.2...
Enter release notes (ctrl-d when done):
Bug fixes and performance improvements
^D

✅ Tag v1.2.2 created

▶ Running cargo publish dry-run...
✅ Dry-run successful

Ready to publish to crates.io
Publish to crates.io? (y/n) y

▶ Publishing to crates.io...
✅ Published to crates.io

Ready to push to GitHub
Push commit and tag to GitHub? (y/n) y

▶ Pushing to GitHub...
✅ Pushed to GitHub

Create GitHub release?
Use gh CLI to create release? (y/n) y

▶ Creating GitHub release...
Enter release title (or press enter for default):
v1.2.2: Bug Fixes
✅ GitHub release created
ℹ️  View at: https://github.com/daneb/omniscient/releases/tag/v1.2.2

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🎉 Release v1.2.2 Complete!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Summary:
  Version: v1.2.2
  Cargo.toml: ✅ Updated
  Tests: ✅ Passed
  Clippy: ✅ Passed
  crates.io: ✅ Published
  GitHub: ✅ Pushed

Next steps:
  • Wait 2-3 minutes for crates.io indexing
  • Test install: cargo install omniscient --version 1.2.2
  • Update CHANGELOG.md if needed
  • Announce on social media
  • Post to Hacker News (see docs/HACKERNEWS_ANNOUNCEMENT.md)

✨ Done!
```

## Manual Steps (If Script Fails)

If you need to do it manually:

```bash
# 1. Run tests and checks
cargo test --verbose
cargo clippy -- -D warnings
cargo fmt -- --check

# 2. Update version
# Edit Cargo.toml manually
cargo build --release  # Updates Cargo.lock

# 3. Commit version bump
git add Cargo.toml Cargo.lock
git commit -m "Bump version to 1.2.2"

# 4. Create tag
git tag -a v1.2.2 -m "Release notes here"

# 5. Publish to crates.io
cargo publish --dry-run  # Test first
cargo publish

# 6. Push to GitHub
git push origin master
git push origin v1.2.2

# 7. Create GitHub release
gh release create v1.2.2 \
  --title "v1.2.2: Title" \
  --notes "Release notes"
```

## Troubleshooting

### "Working directory has uncommitted changes"
```bash
git status
git add .
git commit -m "Prepare for release"
./release.sh 1.2.2
```

### "Tests failed"
Fix the failing tests before releasing.

### "Clippy found warnings"
Fix warnings with:
```bash
cargo clippy --fix
```

### "crate already published"
You can't republish the same version. Bump version and try again:
```bash
./release.sh 1.2.3
```

### "gh CLI not installed"
Install with:
```bash
brew install gh
gh auth login
```

Or create release manually on GitHub web UI.

### "Using work email"
The script will warn you. Either:
- Continue anyway (if intentional)
- Fix with: `git config --local user.email "happyfrog@tuta.io"`
- Or verify account: `git-account`

## Skipping Steps

You can say "n" to any confirmation:
- Skip crates.io publish → You can run `cargo publish` later
- Skip GitHub push → You can run `git push` later
- Skip GitHub release → Create manually on GitHub

The script is safe and lets you control each step.

## Version Numbering

Follow semantic versioning:
- **Patch (1.2.X)**: Bug fixes, docs, minor changes
- **Minor (1.X.0)**: New features, non-breaking changes
- **Major (X.0.0)**: Breaking changes

## Files Modified by Script

- `Cargo.toml` - Version number updated
- `Cargo.lock` - Updated to match Cargo.toml
- Git history - Commit added with version bump
- Git tags - New annotated tag created

## After Release

1. Wait 2-3 minutes for crates.io to index
2. Test installation: `cargo install omniscient --version <VERSION>`
3. Update CHANGELOG.md if you haven't already
4. Announce:
   - Social media
   - Hacker News (see [docs/HACKERNEWS_ANNOUNCEMENT.md](docs/HACKERNEWS_ANNOUNCEMENT.md))
   - Reddit: r/rust, r/commandline
   - This Week in Rust newsletter

---

**Pro tip**: Keep this file open in a tab. When you need to release, just:
```bash
./release.sh 1.2.X
```

And follow the prompts. The script handles everything else.
