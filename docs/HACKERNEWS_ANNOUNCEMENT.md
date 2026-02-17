# Hacker News Announcement: Omniscient v1.2.0

## Title

Omniscient – Never forget a CLI command again (AI-assisted, 100% open source)

## Post

I built Omniscient, a command history tracker for your shell. It captures every command you run, categorizes them intelligently, and makes them instantly searchable across machine migrations.

**What it does:**

- Captures every shell command with zero overhead (<10ms)
- Auto-categorizes by command type (git, docker, npm, etc.)
- Full-text search through your entire history
- NEW in v1.2.0: Contextual queries - filter commands by directory (`omniscient here`)
- Export/import for syncing across machines
- Privacy-first: automatic redaction of passwords/tokens

**Example:**

```bash
# What git commands did I use in this project?
omniscient here --recursive | grep git

# How did I deploy the backend last time?
omniscient search deploy --dir ~/services/backend

# What's my most-used docker command?
omniscient top 10 | grep docker
```

**The AI elephant in the room:**

Yes, this was built with AI assistance (Claude). I know that raises trust questions for a tool that captures your command history. Here's how I addressed that:

1. **100% Open Source**: Every line is auditable (https://github.com/daneb/omniscient)
2. **91 Automated Tests**: Comprehensive test coverage
3. **Zero Network Calls**: Grep the source yourself - all data stays local
4. **Security Audits**: cargo-audit on every commit
5. **Architecture Decision Records**: All major decisions documented
6. **Human Review**: Every AI-generated line was reviewed and validated

The AI helped with implementation, but humans made all architectural decisions, wrote tests, and validated everything. Full transparency: CONTRIBUTING.md documents the development process.

**Tech stack:** Rust, SQLite FTS5, zero dependencies on external services

**Privacy:** Everything stored in `~/.omniscient/`. No telemetry, no analytics, no network calls. You can audit the code yourself.

GitHub: https://github.com/daneb/omniscient

Feedback and contributions welcome!

---

## Expected Questions & Answers

### Q: "How is this different from `history` or `atuin`?"

**A:** Great question!

- **vs `history`**: Survives machine migrations, smart categorization, full-text search, cross-session persistence
- **vs `atuin`**: Privacy-first (no sync server), simpler architecture, contextual queries by directory, open development process with ADRs

Atuin is excellent if you want cloud sync. Omniscient is for those who prefer local-only with manual Git-based sync.

### Q: "Can I trust code written by AI?"

**A:** Valid concern. Here's how I addressed it:

1. **Audit it yourself**: Clone the repo, run tests, grep for network calls
2. **Test coverage**: 91 tests validate functionality
3. **Incremental commits**: Git history shows human-guided development
4. **Architecture decisions**: Humans made all design choices (documented in /docs/adr/)
5. **Security**: cargo-audit runs on every commit (zero vulnerabilities)

AI is a productivity multiplier, not a replacement for human judgment. Every line was reviewed.

### Q: "What about privacy? You're storing all my commands!"

**A:**

- All data in `~/.omniscient/` (local SQLite database)
- Zero network calls (verify: `grep -r "http" src/`)
- Auto-redaction of passwords/tokens/secrets (configurable)
- Commands matching sensitive patterns are dropped entirely (not stored)
- Export/import gives you full control

Run this yourself:
```bash
git clone https://github.com/daneb/omniscient
cd omniscient
grep -r "std::net" src/  # Should return nothing
grep -r "reqwest" Cargo.toml  # Should return nothing
```

### Q: "Why Rust?"

**A:**

- **Performance**: Sub-10ms capture overhead
- **Safety**: Type system prevents common bugs
- **Single Binary**: Easy distribution
- **SQLite Integration**: Excellent rusqlite library
- **Cross-platform**: Works on Linux, macOS, Windows (with WSL)

### Q: "How do you handle the database getting huge?"

**A:**

- SQLite handles millions of rows efficiently
- FTS5 (full-text search) keeps searches fast
- Deduplication: Increments usage_count instead of storing duplicates
- Export old data and start fresh if needed
- Typical user: ~50k commands = ~10MB database

### Q: "What's the roadmap?"

**A:**

v1.3 (next):
- Fish shell support
- Stats by directory
- Path-based analytics

v2.0 (future):
- Workspace awareness
- Command suggestions based on context
- Optional cloud sync (self-hosted)

See: https://github.com/daneb/omniscient/blob/master/docs/planning/roadmap.md

### Q: "How can I contribute?"

**A:**

Contributions welcome! See CONTRIBUTING.md

Areas where help would be appreciated:
- Fish shell integration
- Windows native support (currently WSL only)
- Performance optimization for huge histories (1M+ commands)
- Additional categorization patterns
- UI/TUI interface

### Q: "License?"

**A:** MIT - use it however you want.

---

## Alternative Shorter Version (if character limit)

**Title:** Omniscient – CLI command history tracker with contextual queries

**Post:**

Built a command history tracker in Rust. Captures every shell command, categorizes automatically, full-text search, syncs across machines.

New in v1.2: Contextual queries - `omniscient here` shows commands run in current directory.

**Addressing the trust question:** Yes, built with AI assistance. How you can trust it:

- 100% open source: https://github.com/daneb/omniscient
- 91 automated tests, zero clippy warnings
- Zero network calls (grep the source yourself)
- cargo-audit security checks
- All architectural decisions documented (ADRs)

All data stored locally in ~/.omniscient/. No telemetry, no cloud.

Tech: Rust, SQLite FTS5. Works on Linux/macOS/WSL.

Feedback welcome!

---

## Posting Strategy

### Timing
- Tuesday-Thursday, 8-10 AM EST (peak HN traffic)
- Avoid Mondays (slow) and Fridays (drops fast)

### Engagement Plan
- Monitor first 30 minutes closely
- Respond to questions promptly and transparently
- Be humble, not defensive
- Acknowledge valid concerns
- Link to specific code/tests when relevant

### Key Points to Emphasize
1. **Transparency**: Open source, documented decisions
2. **Verification**: "Don't trust me, audit the code"
3. **Privacy**: Local-only, zero network calls
4. **Quality**: Tests, security audits, CI/CD
5. **AI Transparency**: Clear about AI assistance + human oversight

### Don't
- ❌ Oversell features
- ❌ Get defensive about AI
- ❌ Claim it's "production-ready for everyone" (let users decide)
- ❌ Compare negatively to other tools
- ❌ Ignore legitimate concerns

### Do
- ✅ Be honest about limitations
- ✅ Welcome code audits
- ✅ Acknowledge AI assistance upfront
- ✅ Invite contributions
- ✅ Provide concrete verification steps
- ✅ Share architecture decisions (ADRs)

---

## Follow-up Comments Template

**If someone raises security concerns:**

"Great question. Here's how you can verify [specific concern]:

```bash
[concrete verification command]
```

I documented the security architecture in SECURITY.md: [link]

What specific aspects would you like me to clarify?"

**If someone asks about AI:**

"You're right to be skeptical. Here's what I did to ensure quality:

1. [specific measure]
2. [specific test/audit]
3. [specific documentation]

The full development process is documented in CONTRIBUTING.md. Would you like me to point you to specific tests or architectural decisions?"

**If someone suggests improvements:**

"That's a great idea! Would you be interested in contributing? I've documented how to [relevant section in CONTRIBUTING.md].

If not, I'll add it to the roadmap: [GitHub issue link]"

---

## Success Metrics

**Good Response:**
- Constructive technical discussion
- Questions about implementation
- Contributions or stars
- Security researchers auditing code

**Great Response:**
- Front page for >4 hours
- Multiple pull requests
- Detailed technical discussions
- "I audited the code and here's what I found" comments

**Best Response:**
- Thoughtful critique leading to improvements
- Security researchers finding and responsibly disclosing issues
- Community contributions
- Blog posts/articles analyzing the architecture

---

## Post-Launch Checklist

- [ ] Monitor HN comments for first 6 hours
- [ ] Respond to all questions within 2 hours
- [ ] Create GitHub issues for feature requests
- [ ] Thank contributors
- [ ] Update FAQ if common questions emerge
- [ ] Write retrospective blog post
