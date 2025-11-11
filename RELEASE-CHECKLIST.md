# Release Checklist - v1.0.0

Use this checklist to ensure a smooth release process.

## 🔍 Pre-Release Verification

### Code Quality
- [x] All tests passing (`cargo test`)
- [x] Zero compiler warnings (`cargo build`)
- [x] Zero clippy warnings (`cargo clippy`)
- [x] Code formatted (`cargo fmt --check`)

### Documentation
- [x] README.md updated with v1.0.0 info
- [x] CHANGELOG.md has all changes listed
- [x] RELEASE-NOTES.md created
- [x] CONTRIBUTING.md current
- [x] All examples working
- [x] Documentation organized in docs/

### Project Files
- [x] Cargo.toml version is 1.0.0
- [x] Cargo.toml metadata complete
- [x] LICENSE file present (MIT)
- [x] .gitignore configured
- [x] No sensitive data in repo

### Functionality
- [x] Build successful (`cargo build --release`)
- [x] Binary size acceptable (5.2MB < 10MB target)
- [x] All commands work (init, capture, search, etc.)
- [x] Integration test passes
- [x] Performance meets targets

## 📝 Release Process

### 1. Final Commit
- [ ] Stage all changes: `git add .`
- [ ] Review changes: `git status`
- [ ] Create commit: `git commit -m "Release v1.0.0"`

### 2. Create Tag
- [ ] Create annotated tag: `git tag -a v1.0.0 -m "Message"`
- [ ] Verify tag: `git tag -l`
- [ ] Check tag details: `git show v1.0.0`

### 3. Push to Remote
- [ ] Push commits: `git push origin master`
- [ ] Push tag: `git push origin v1.0.0`
- [ ] Verify on GitHub: Check tags/releases page

### 4. GitHub Release
- [ ] Go to GitHub Releases
- [ ] Click "Draft a new release"
- [ ] Select tag v1.0.0
- [ ] Title: "Omniscient v1.0.0 - First Stable Release"
- [ ] Copy content from RELEASE-NOTES.md
- [ ] Check "This is the latest release"
- [ ] Publish release

## 🚀 Post-Release

### Verification
- [ ] Tag visible on GitHub
- [ ] Release page created
- [ ] Download links work
- [ ] README renders correctly
- [ ] Documentation accessible

### Optional
- [ ] Tweet announcement
- [ ] Post to Reddit (r/rust, r/commandline)
- [ ] Share on HackerNews
- [ ] Update personal website/portfolio
- [ ] Announce in Discord/Slack communities

### Cargo (When Ready)
- [ ] Test: `cargo publish --dry-run`
- [ ] Publish: `cargo publish`
- [ ] Verify on crates.io

## 📊 Metrics to Track

After release, monitor:
- [ ] GitHub stars/forks
- [ ] Issue reports
- [ ] Pull requests
- [ ] Download counts (if applicable)
- [ ] User feedback

## 🔧 Troubleshooting

If something goes wrong:

### Tag Issues
- Delete local tag: `git tag -d v1.0.0`
- Delete remote tag: `git push origin :refs/tags/v1.0.0`
- Recreate tag with corrections

### Commit Issues
- Undo last commit (keep changes): `git reset --soft HEAD^`
- Undo last commit (discard): `git reset --hard HEAD^`

### Push Issues
- Force push (use carefully): `git push -f origin master`
- Force push tag: `git push -f origin v1.0.0`

## ✅ Final Status

**Project**: Omniscient v1.0.0
**Status**: Ready for Release ✨
**Quality**: Production Ready ✅

---

## Quick Commands

```bash
# Easiest: Use the script
./release-tag.sh

# Or manual:
git add .
git commit -m "Release v1.0.0"
git tag -a v1.0.0 -m "First stable release"
git push origin master && git push origin v1.0.0
```

---

**Remember**: Once you push, the tag is public. Make sure everything is ready!

🎉 Good luck with your release!
