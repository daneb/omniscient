# Automatic Git Account Switching

**Problem**: Accidentally using work email for personal projects (privacy/security risk)
**Solution**: Automatic account switching based on directory + verification tool

## ✅ What's Now Configured

### Automatic Switching by Directory

Your git now **automatically switches accounts** based on directory:

| Directory | Email | Account |
|-----------|-------|---------|
| `~/Documents/Repos/Personal/` | `happyfrog@tuta.io` | Personal (daneb) |
| `~/Documents/Repos/Work/` | `dane.balia2@standardbank.co.za` | Work (StandardBank) |
| `~/work/` | `dane.balia2@standardbank.co.za` | Work (StandardBank) |
| Everything else | `happyfrog@tuta.io` | Personal (default) |

### Configuration Files

```
~/.gitconfig           # Main config with includeIf directives
~/.gitconfig-personal  # Personal account settings
~/.gitconfig-work      # Work account settings
```

## 🔍 Verify Which Account is Active

### Quick Check

```bash
git-account
# or
git-which-account
```

**Output example:**
```
📧 Git Account Check
====================

Directory: /Users/dane.balia2/Documents/Repos/Personal/omniscient

Active Configuration:
  Name:  Dane Balia
  Email: happyfrog@tuta.io

✅ Personal Account (daneb)
   SSH: git@github.com-dgb

Remote: git@github.com-dgb:daneb/omniscient.git
✅ Remote configured for personal account
```

### Before Every Commit

**Best Practice**: Always verify before your first commit in a new repo:

```bash
cd new-repo
git-account  # ← Check this!
```

## 📁 Directory Structure Best Practices

Organize your repos by account to enable automatic switching:

```
~/Documents/Repos/
├── Personal/          # ← Automatically uses happyfrog@tuta.io
│   ├── omniscient/
│   ├── my-blog/
│   └── side-project/
└── Work/              # ← Automatically uses dane.balia2@standardbank.co.za
    ├── internal-tool/
    └── company-project/
```

## 🚨 Security Risks Avoided

### ✅ What You Prevented

1. **Work email in personal projects** - Could expose:
   - Your employer's name
   - Work email to scrapers/spam
   - Potential NDA violations

2. **Personal email in work repos** - Could cause:
   - Audit trail confusion
   - Corporate compliance issues
   - IP ownership questions

3. **Mixed commit history** - Makes it harder to:
   - Separate work/personal contributions
   - Maintain professional boundaries
   - Export resume/portfolio

## 🔧 Manual Override (If Needed)

### Force Personal Account

```bash
git config --local user.email "happyfrog@tuta.io"
```

### Force Work Account

```bash
git config --local user.email "dane.balia2@standardbank.co.za"
```

### Verify

```bash
git-account
```

## 🛠️ Fixing Existing Repos

### If You Find Wrong Email in a Repo

```bash
cd wrong-repo
git-account  # Verify the issue

# Fix it
git config --local user.email "happyfrog@tuta.io"

# Verify fix
git-account
```

### Rewrite Recent Commits (Before Pushing)

```bash
# Rewrite last N commits with correct email
git rebase -i HEAD~3 -x "git commit --amend --author='Dane Balia <happyfrog@tuta.io>' --no-edit"

# Only if not yet pushed to remote
git push --force-with-lease
```

⚠️ **Warning**: Only do this on commits you haven't shared with others!

## 📋 Pre-Commit Checklist

**Every new repository:**

1. ✅ Clone to correct directory (`Personal/` or `Work/`)
2. ✅ Run `git-account` to verify
3. ✅ Check remote URL matches account
4. ✅ Make first commit
5. ✅ Push and verify on GitHub

## 🔐 SSH Key Mapping

| Account | SSH Host | Key | GitHub User |
|---------|----------|-----|-------------|
| Personal | `github.com-dgb` | `~/.ssh/id_rsa_github_personal` | `daneb` |
| Work | `github.com` | `~/.ssh/id_rsa` | (your work account) |

### Clone Commands

**Personal:**
```bash
git clone git@github.com-dgb:daneb/repo.git ~/Documents/Repos/Personal/repo
```

**Work:**
```bash
git clone git@github.com:Company/repo.git ~/Documents/Repos/Work/repo
```

## 🧪 Testing the Setup

### Test Personal Account

```bash
cd ~/Documents/Repos/Personal/
mkdir test-personal
cd test-personal
git init
git-account
# Should show: ✅ Personal Account (happyfrog@tuta.io)
```

### Test Work Account

```bash
cd ~/Documents/Repos/Work/
mkdir test-work
cd test-work
git init
git-account
# Should show: ⚠️ Work Account (dane.balia2@standardbank.co.za)
```

## 🎯 Quick Reference

```bash
# Check current account
git-account

# View configuration
git config --list | grep user

# See which config file is being used
git config --show-origin user.email

# Test SSH for personal
ssh -T git@github.com-dgb

# Test SSH for work
ssh -T git@github.com
```

## 📝 Adding to Your Workflow

### Add to Your Shell Profile

Already added to `~/.zshrc`:
```bash
export PATH="$HOME/bin:$PATH"
alias git-account='git-which-account'
```

Reload:
```bash
source ~/.zshrc
```

### Git Pre-Commit Hook (Optional)

Create `.git/hooks/pre-commit` in any repo to enforce checks:

```bash
#!/bin/bash
# Verify correct email before committing

EMAIL=$(git config user.email)
REMOTE=$(git remote get-url origin 2>/dev/null || echo "")

# Personal repo check
if [[ "$REMOTE" == *"daneb"* ]] && [[ "$EMAIL" != *"happyfrog"* ]]; then
    echo "❌ ERROR: Using wrong email for personal repo!"
    echo "Current: $EMAIL"
    echo "Expected: happyfrog@tuta.io"
    echo ""
    echo "Fix with: git config --local user.email happyfrog@tuta.io"
    exit 1
fi

# Work repo check
if [[ "$REMOTE" == *"StandardBank"* ]] && [[ "$EMAIL" != *"standardbank"* ]]; then
    echo "❌ ERROR: Using wrong email for work repo!"
    echo "Current: $EMAIL"
    echo "Expected: dane.balia2@standardbank.co.za"
    echo ""
    echo "Fix with: git config --local user.email dane.balia2@standardbank.co.za"
    exit 1
fi
```

## ✅ You're Protected Now!

The automatic switching is now active. Just:

1. Keep personal projects in `~/Documents/Repos/Personal/`
2. Keep work projects in `~/Documents/Repos/Work/`
3. Run `git-account` when in doubt
4. Your email will be correct automatically! 🎉

---

**Current Status for omniscient:**
```
✅ Email: happyfrog@tuta.io
✅ Remote: git@github.com-dgb:daneb/omniscient.git
✅ SSH: Authenticated as daneb
```

You're good to publish! 🚀
