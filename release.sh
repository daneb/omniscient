#!/bin/bash
# Complete release workflow for Omniscient
# Usage: ./release.sh <version>
# Example: ./release.sh 1.2.2

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

step() {
    echo -e "\n${BLUE}▶ $1${NC}"
}

# Check arguments
if [ $# -ne 1 ]; then
    error "Usage: ./release.sh <version>\nExample: ./release.sh 1.2.2"
fi

VERSION=$1
VERSION_TAG="v${VERSION}"

echo -e "${BLUE}🚀 Omniscient Release Workflow${NC}"
echo "================================"
echo -e "Version: ${GREEN}${VERSION_TAG}${NC}\n"

# Check if we're in a git repo
if [ ! -d .git ]; then
    error "Not in a git repository"
fi

# Check if Cargo.toml exists
if [ ! -f Cargo.toml ]; then
    error "Cargo.toml not found"
fi

# Check git account
step "Checking git account..."
GIT_EMAIL=$(git config user.email)
if [[ "$GIT_EMAIL" == *"standardbank"* ]]; then
    warning "Using work email: $GIT_EMAIL"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        error "Aborted. Run 'git-account' to verify configuration."
    fi
else
    success "Git account: $GIT_EMAIL"
fi

# Check if tag already exists (locally or remotely)
step "Checking if release already exists..."
TAG_EXISTS_LOCAL=false
TAG_EXISTS_REMOTE=false

if git rev-parse "$VERSION_TAG" >/dev/null 2>&1; then
    TAG_EXISTS_LOCAL=true
fi

if git ls-remote --tags origin 2>/dev/null | grep -q "refs/tags/$VERSION_TAG"; then
    TAG_EXISTS_REMOTE=true
fi

if [ "$TAG_EXISTS_LOCAL" = true ] || [ "$TAG_EXISTS_REMOTE" = true ]; then
    error "Tag $VERSION_TAG already exists (local: $TAG_EXISTS_LOCAL, remote: $TAG_EXISTS_REMOTE)"
fi
success "Tag $VERSION_TAG does not exist yet"

# Check working directory is clean
step "Checking git status..."
if ! git diff-index --quiet HEAD --; then
    error "Working directory has uncommitted changes. Commit or stash them first."
fi
success "Working directory is clean"

# Run tests
step "Running tests..."
if ! cargo test --verbose; then
    error "Tests failed"
fi
success "All tests passed"

# Run clippy
step "Running clippy..."
if ! cargo clippy -- -D warnings; then
    error "Clippy found warnings"
fi
success "No clippy warnings"

# Check formatting
step "Checking formatting..."
if ! cargo fmt -- --check; then
    warning "Code needs formatting. Run 'cargo fmt' to fix."
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        error "Aborted"
    fi
else
    success "Code is formatted"
fi

# Update version in Cargo.toml
step "Updating Cargo.toml version..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
else
    # Linux
    sed -i "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
fi

# Verify version was updated
TOML_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
if [ "$TOML_VERSION" != "$VERSION" ]; then
    error "Failed to update Cargo.toml version"
fi
success "Updated Cargo.toml to version ${VERSION}"

# Update Cargo.lock (use check instead of build to avoid security software issues)
step "Updating Cargo.lock..."
if cargo check > /dev/null 2>&1; then
    success "Cargo.lock updated"
else
    warning "Cargo check had issues, but continuing (cargo publish will verify)"
fi

# Show what will be committed
echo -e "\n${BLUE}Changes to commit:${NC}"
git diff Cargo.toml
git diff Cargo.lock
echo

# Confirm version bump
read -p "Commit version bump to ${VERSION_TAG}? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    # Restore Cargo.toml
    git checkout Cargo.toml Cargo.lock
    error "Aborted"
fi

# Check if version already bumped in recent commits
if git log -5 --pretty=%B | grep -q "Bump version to ${VERSION}"; then
    warning "Version ${VERSION} already bumped in recent commits"
    info "Skipping commit step (idempotent re-run)"
else
    # Commit version bump
    step "Committing version bump..."
    git add Cargo.toml Cargo.lock
    git commit -m "Bump version to ${VERSION}

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
    success "Version bump committed"
fi

# Create git tag
step "Creating git tag ${VERSION_TAG}..."
echo -e "${YELLOW}Enter release notes (ctrl-d when done):${NC}"
TAG_MESSAGE=$(cat)

if [ -z "$TAG_MESSAGE" ]; then
    warning "No tag message provided, using default"
    TAG_MESSAGE="Release ${VERSION_TAG}"
fi

git tag -a "$VERSION_TAG" -m "$TAG_MESSAGE"
success "Tag ${VERSION_TAG} created"

# Show tag
git show "$VERSION_TAG" --no-patch

# Cargo publish dry run
step "Running cargo publish dry-run..."
if ! cargo publish --dry-run; then
    error "Cargo publish dry-run failed"
fi
success "Dry-run successful"

# Check if already published to crates.io
step "Checking crates.io..."
if cargo search omniscient --limit 1 2>/dev/null | grep -q "omniscient = \"${VERSION}\""; then
    warning "Version ${VERSION} already published to crates.io"
    info "Skipping publish step (idempotent re-run)"
    PUBLISHED_CARGO=true
else
    # Confirm publish
    echo -e "\n${YELLOW}Ready to publish to crates.io${NC}"
    read -p "Publish to crates.io? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        step "Publishing to crates.io..."
        if cargo publish; then
            success "Published to crates.io"
            PUBLISHED_CARGO=true
        else
            error "Cargo publish failed"
        fi
    else
        info "Skipped crates.io publish"
        PUBLISHED_CARGO=false
    fi
fi

# Push to remote
echo -e "\n${YELLOW}Ready to push to GitHub${NC}"
read -p "Push commit and tag to GitHub? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    step "Pushing to GitHub..."
    git push origin master
    git push origin "$VERSION_TAG"
    success "Pushed to GitHub"
    PUSHED_GIT=true
else
    info "Skipped GitHub push"
    PUSHED_GIT=false
fi

# Create GitHub release
if [ "$PUSHED_GIT" = true ]; then
    echo -e "\n${YELLOW}Create GitHub release?${NC}"
    read -p "Use gh CLI to create release? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Check if gh is installed
        if ! command -v gh &> /dev/null; then
            warning "gh CLI not installed. Install: brew install gh"
            info "Or create release manually: https://github.com/daneb/omniscient/releases/new"
        else
            # Check if release already exists
            if gh release view "$VERSION_TAG" &>/dev/null; then
                success "GitHub release already exists for ${VERSION_TAG}"
                info "View at: https://github.com/daneb/omniscient/releases/tag/${VERSION_TAG}"
                info "Skipping release creation (idempotent re-run)"
            else
                step "Creating GitHub release..."
                echo -e "${YELLOW}Enter release title (or press enter for default):${NC}"
                read -r RELEASE_TITLE
                if [ -z "$RELEASE_TITLE" ]; then
                    RELEASE_TITLE="${VERSION_TAG}"
                fi

                # Create release with tag message
                RELEASE_OUTPUT=$(echo "$TAG_MESSAGE" | gh release create "$VERSION_TAG" \
                    --title "$RELEASE_TITLE" \
                    --notes-file - 2>&1)
                RELEASE_EXIT=$?

                if [ $RELEASE_EXIT -eq 0 ]; then
                    success "GitHub release created"
                    info "View at: https://github.com/daneb/omniscient/releases/tag/${VERSION_TAG}"
                else
                    warning "Failed to create GitHub release"

                    # Check if it's a workflow scope issue
                    if echo "$RELEASE_OUTPUT" | grep -q "workflow.*scope"; then
                        info "Missing 'workflow' scope. Run: gh auth refresh -h github.com -s workflow"
                        info "Then create release: gh release create ${VERSION_TAG} --title \"${RELEASE_TITLE}\" --notes \"...\""
                    fi

                    info "Or create manually: https://github.com/daneb/omniscient/releases/new?tag=${VERSION_TAG}"
                fi
            fi
        fi
    fi
fi

# Summary
echo -e "\n${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}🎉 Release ${VERSION_TAG} Complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

echo "Summary:"
echo "  Version: ${VERSION_TAG}"
echo "  Cargo.toml: ✅ Updated"
echo "  Tests: ✅ Passed"
echo "  Clippy: ✅ Passed"

if [ "$PUBLISHED_CARGO" = true ]; then
    echo "  crates.io: ✅ Published"
    info "Check: https://crates.io/crates/omniscient"
else
    echo "  crates.io: ⏭️  Skipped"
    info "Publish later: cargo publish"
fi

if [ "$PUSHED_GIT" = true ]; then
    echo "  GitHub: ✅ Pushed"
    info "View: https://github.com/daneb/omniscient"
else
    echo "  GitHub: ⏭️  Skipped"
    info "Push later: git push origin master && git push origin ${VERSION_TAG}"
fi

echo -e "\n${BLUE}Next steps:${NC}"
if [ "$PUBLISHED_CARGO" = true ]; then
    echo "  • Wait 2-3 minutes for crates.io indexing"
    echo "  • Test install: cargo install omniscient --version ${VERSION}"
fi
if [ "$PUSHED_GIT" = true ]; then
    echo "  • Update CHANGELOG.md if needed"
    echo "  • Announce on social media"
    echo "  • Post to Hacker News (see docs/HACKERNEWS_ANNOUNCEMENT.md)"
fi

echo -e "\n${GREEN}✨ Done!${NC}\n"
