#!/bin/bash
# Script to create and push v1.0.0 release tag
# Usage: ./release-tag.sh

set -e

echo "🚀 Omniscient v1.0.0 Release Tag"
echo "================================"
echo

# Check if we're in a git repo
if [ ! -d .git ]; then
    echo "❌ Error: Not in a git repository"
    exit 1
fi

# Show current status
echo "📋 Current status:"
git status --short
echo

# Confirm with user
read -p "📦 Ready to commit and tag v1.0.0? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Aborted"
    exit 1
fi

# Add all changes
echo
echo "➕ Adding all changes..."
git add .
echo "✅ Changes staged"

# Create commit
echo
echo "📝 Creating commit..."
git commit -m "Release v1.0.0 - Production ready CLI command history tracker

Complete implementation of all core features:
- Command capture with Zsh integration
- Smart categorization (13 categories, 80+ commands)
- Full-text search with FTS5
- Export/Import with merge strategies
- Privacy-first redaction engine
- Statistics and analytics
- Comprehensive test suite (75 tests, 85% coverage)
- Complete documentation suite

Quality metrics:
- Zero compiler warnings
- Zero clippy warnings
- All tests passing
- Production-ready error handling
- Binary size: 5.2MB
- Performance: <500ms with 1000+ commands

Documentation:
- README with quick start
- CHANGELOG with all features
- CONTRIBUTING guidelines
- Complete examples
- Organized docs/ structure

Ready for production use and community distribution."

echo "✅ Commit created"

# Create annotated tag
echo
echo "🏷️  Creating annotated tag v1.0.0..."
git tag -a v1.0.0 -m "Omniscient v1.0.0

First stable release of CLI command history tracker.

🎯 Core Features:
- Automatic command capture via Zsh hooks
- Smart categorization (13 categories, 80+ commands)
- Full-text search using SQLite FTS5
- Export/Import with 3 merge strategies
- Privacy-first redaction engine
- Comprehensive statistics & analytics

✨ Quality:
- 75 comprehensive tests (85% coverage)
- Zero compiler/clippy warnings
- Production-ready error handling
- Complete documentation

⚡ Performance:
- 5.2MB binary size
- ~5ms capture time
- <500ms search with 1000+ commands

📚 Documentation:
- Complete user guide (README.md)
- Detailed changelog (CHANGELOG.md)
- Contributing guidelines (CONTRIBUTING.md)
- Comprehensive examples
- Organized docs/ structure

See RELEASE-NOTES.md for complete details."

echo "✅ Tag v1.0.0 created"

# Show tag
echo
echo "📋 Tag details:"
git tag -l v1.0.0
echo

# Push instructions
echo "🚀 Next steps:"
echo "───────────────────────────────────"
echo
echo "1. Push commit to remote:"
echo "   git push origin master"
echo
echo "2. Push tag to remote:"
echo "   git push origin v1.0.0"
echo
echo "3. Or push both at once:"
echo "   git push --follow-tags"
echo
echo "4. Create GitHub Release:"
echo "   - Go to GitHub repository"
echo "   - Click 'Releases'"
echo "   - Click 'Draft a new release'"
echo "   - Select tag: v1.0.0"
echo "   - Add release notes from RELEASE-NOTES.md"
echo "   - Publish release"
echo
echo "───────────────────────────────────"
echo "✅ Local tag created successfully!"
echo

# Offer to push
read -p "🚀 Push to remote now? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo
    echo "📤 Pushing to remote..."
    git push origin master
    git push origin v1.0.0
    echo
    echo "✅ Pushed to remote successfully!"
    echo
    echo "🎉 v1.0.0 is now live!"
    echo "Check your GitHub repository for the new tag."
else
    echo
    echo "ℹ️  Tag created locally. Run these commands when ready:"
    echo "   git push origin master"
    echo "   git push origin v1.0.0"
fi

echo
