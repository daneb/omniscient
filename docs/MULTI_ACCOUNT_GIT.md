# Managing Multiple GitHub Accounts

Quick reference for working with multiple GitHub accounts on the same machine.

## Current Setup

You have two GitHub accounts configured:

1. **Personal** (`daneb`) - Uses `github.com-dgb`
2. **Work** (StandardBank) - Uses default `github.com`

## SSH Configuration

Your `~/.ssh/config` should have:

```ssh
# Personal GitHub (daneb)
Host github.com-dgb
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_rsa_github_personal

# Work GitHub (StandardBank)
Host github.com
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_rsa
```

## Repository Configuration

### For Personal Repos (like omniscient)

```bash
# Set local user for this repo
git config --local user.name "Dane Balia"
git config --local user.email "happyfrog@tuta.io"

# Remote should use the -dgb host
git remote add origin git@github.com-dgb:daneb/omniscient.git
```

### For Work Repos

```bash
# Set local user for work repo
git config --local user.name "Dane Balia"
git config --local user.email "dane.balia2@standardbank.co.za"

# Remote uses default github.com
git remote add origin git@github.com:StandardBank/repo.git
```

## Quick Commands

### Check Current Configuration

```bash
# In any repo
git config user.email  # What email will be used?
git remote -v          # Which host is configured?
```

### Test SSH Connection

```bash
# Test personal account
ssh -T git@github.com-dgb
# Should say: "Hi daneb!"

# Test work account
ssh -T git@github.com
# Should say: "Hi StandardBank-username!"
```

### Fix Wrong Configuration

```bash
# If you're in wrong account:
git config --local user.email "happyfrog@tuta.io"  # Personal
# or
git config --local user.email "dane.balia2@standardbank.co.za"  # Work

# If remote is wrong:
git remote set-url origin git@github.com-dgb:daneb/repo.git  # Personal
# or
git remote set-url origin git@github.com:org/repo.git  # Work
```

## Clone Repositories

### Personal Repos

```bash
git clone git@github.com-dgb:daneb/omniscient.git
cd omniscient
git config --local user.email "happyfrog@tuta.io"
```

### Work Repos

```bash
git clone git@github.com:StandardBank/repo.git
cd repo
git config --local user.email "dane.balia2@standardbank.co.za"
```

## Troubleshooting

### "Permission denied (publickey)"

Check which key is being used:
```bash
ssh -vT git@github.com-dgb 2>&1 | grep "identity file"
```

### "Author email does not match GitHub account"

Check and fix:
```bash
git config user.email  # Check current
git config --local user.email "happyfrog@tuta.io"  # Fix
```

### "Remote repository not found"

Check the host in remote URL:
```bash
git remote -v
# Personal should have: github.com-dgb
# Work should have: github.com
```

## Automatic Configuration (Optional)

You can automate this with git directory-based config:

```bash
# Add to ~/.gitconfig

[includeIf "gitdir:~/Documents/Repos/Personal/"]
    path = ~/.gitconfig-personal

[includeIf "gitdir:~/Documents/Repos/Work/"]
    path = ~/.gitconfig-work
```

Then create:

**~/.gitconfig-personal**:
```toml
[user]
    name = Dane Balia
    email = happyfrog@tuta.io
```

**~/.gitconfig-work**:
```toml
[user]
    name = Dane Balia
    email = dane.balia2@standardbank.co.za
```

This way, repos in `~/Documents/Repos/Personal/` automatically use personal email!

## Current Status (omniscient repo)

✅ **Correctly configured for personal account:**

```
User:   Dane Balia
Email:  happyfrog@tuta.io
Remote: git@github.com-dgb:daneb/omniscient.git
SSH:    Authenticated as daneb
```

You're good to go! 🚀
