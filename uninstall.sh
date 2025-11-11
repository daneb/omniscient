#!/usr/bin/env bash

# Omniscient Uninstallation Script
# Removes omniscient binary, configuration, and shell hooks

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Omniscient Uninstaller                  ║${NC}"
echo -e "${BLUE}╔══════════════════════════════════════════╗${NC}"
echo ""

# Function to print messages
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Detect shell configuration file
detect_shell_rc() {
    if [ -n "$ZSH_VERSION" ]; then
        echo "$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        if [ "$(uname)" = "Darwin" ]; then
            echo "$HOME/.bash_profile"
        else
            echo "$HOME/.bashrc"
        fi
    else
        echo "$HOME/.profile"
    fi
}

SHELL_RC=$(detect_shell_rc)

echo -e "This will uninstall Omniscient from your system.\n"
echo "The following will be removed/modified:"
echo "  1. Binary from ~/.cargo/bin/omniscient"
echo "  2. Configuration directory at ~/.omniscient"
echo "  3. Shell hooks from $SHELL_RC"
echo ""

# Ask for confirmation
read -p "Do you want to proceed? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_warning "Uninstallation cancelled."
    exit 0
fi

echo ""

# Step 1: Remove shell hooks
print_info "Removing shell hooks from $SHELL_RC..."

if [ -f "$SHELL_RC" ]; then
    # Create backup
    BACKUP_FILE="${SHELL_RC}.omniscient.backup.$(date +%Y%m%d_%H%M%S)"
    cp "$SHELL_RC" "$BACKUP_FILE"
    print_info "Created backup at $BACKUP_FILE"

    # Remove omniscient section
    if grep -q "# Omniscient - Command History Tracker" "$SHELL_RC"; then
        # Remove from "# Omniscient" comment to the end of the shell hooks
        sed -i.tmp '/# Omniscient - Command History Tracker/,/# compinit/d' "$SHELL_RC"
        # Also remove any standalone omniscient function calls
        sed -i.tmp '/precmd_functions+=(_omniscient_precmd)/d' "$SHELL_RC"
        sed -i.tmp '/preexec_functions+=(_omniscient_preexec)/d' "$SHELL_RC"
        rm -f "${SHELL_RC}.tmp"
        print_success "Removed shell hooks from $SHELL_RC"
    else
        print_warning "No omniscient hooks found in $SHELL_RC"
    fi
else
    print_warning "$SHELL_RC not found"
fi

# Step 2: Remove binary
print_info "Removing omniscient binary..."

CARGO_BIN="$HOME/.cargo/bin/omniscient"
if [ -f "$CARGO_BIN" ]; then
    rm -f "$CARGO_BIN"
    print_success "Removed binary from $CARGO_BIN"
else
    # Check if installed via cargo
    if command -v cargo >/dev/null 2>&1; then
        if cargo uninstall omniscient 2>/dev/null; then
            print_success "Uninstalled omniscient via cargo"
        else
            print_warning "Binary not found at $CARGO_BIN"
        fi
    else
        print_warning "Binary not found at $CARGO_BIN"
    fi
fi

# Step 3: Ask about data removal
echo ""
print_warning "Your command history data is stored at ~/.omniscient"
read -p "Do you want to delete your command history data? [y/N] " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
    OMNISCIENT_DIR="$HOME/.omniscient"
    if [ -d "$OMNISCIENT_DIR" ]; then
        # Create backup before deletion
        BACKUP_DIR="${HOME}/omniscient_backup_$(date +%Y%m%d_%H%M%S)"
        print_info "Creating backup at $BACKUP_DIR..."
        cp -r "$OMNISCIENT_DIR" "$BACKUP_DIR"
        print_success "Backup created at $BACKUP_DIR"

        # Remove directory
        rm -rf "$OMNISCIENT_DIR"
        print_success "Removed data directory $OMNISCIENT_DIR"
    else
        print_warning "Data directory not found at $OMNISCIENT_DIR"
    fi
else
    print_info "Keeping data directory at ~/.omniscient"
    print_info "You can manually delete it later if needed"
fi

echo ""
echo -e "${GREEN}╔══════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  Uninstallation Complete!               ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════╝${NC}"
echo ""
print_info "To complete the uninstallation, restart your shell or run:"
echo -e "  ${YELLOW}source $SHELL_RC${NC}"
echo ""

if [ -f "$BACKUP_FILE" ]; then
    print_info "Shell config backup: $BACKUP_FILE"
fi

if [ -d "${HOME}/omniscient_backup_"* 2>/dev/null ]; then
    print_info "Data backup: ${HOME}/omniscient_backup_*"
fi

echo ""
print_info "Thank you for using Omniscient! 👋"
