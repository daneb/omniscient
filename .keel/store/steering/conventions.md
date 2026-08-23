---
id: CONV-0001
scope: repo
owner: human
verified_at: 2026-08-23
---

# Conventions

## Working agreement

- Match the surrounding code. Naming, comment density and idiom are local.
- Change the smallest surface that solves the problem.
- A test that mocks away the behaviour under test is worse than no test.

## Rules

- Capture failures MUST be recorded somewhere durable, never only printed —
  the shell hook discards output.
- Shell hook code MUST NOT depend on the user's history options
  (`SHARE_HISTORY`, `HIST_IGNORE_SPACE`, `HIST_IGNORE_DUPS`).
- Every SQLite connection MUST set a busy timeout before first use.
- Changes to `src/shell.rs` MUST state whether existing installs need to re-run
  `omniscient init`.
