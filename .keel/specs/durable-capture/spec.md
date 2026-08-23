---
id: SPEC-0002
slug: durable-capture
schema: keel.spec/1
status: draft
scope:
  - "src/storage.rs"
  - "src/capture.rs"
  - "src/config.rs"
  - "src/main.rs"
  - "src/shell.rs"
  - "tests/**"
  - "CHANGELOG.md"
budget:
  criteria: 7
  lines: 600
verified_at: 2026-08-23
---

# A capture that fails must not fail silently

## Context

**Correction.** This spec was written believing that `storage.rs` never set a
SQLite busy timeout, and that concurrent terminals were therefore losing writes
to `SQLITE_BUSY`. That was wrong: rusqlite defaults `busy_timeout` to 5000 ms,
so the timeout was always in place. The concurrency test written to reproduce
the loss passes with the setting removed, which is how the error was found.

What remains, and is real: **a capture that fails leaves no trace.** On error
`main.rs` writes to stderr and exits 0, and the shell hook runs
`omniscient capture ... &>/dev/null &!` — both streams discarded. Any failure
for any reason is therefore invisible, and there is no way to answer "is this
tool actually recording my commands?" other than noticing an absence months
later.

Found while building the diagnostic: on this machine the history database holds
**zero rows** and no omniscient hook appears in any zsh startup file. A `doctor`
that reported "0 commands, no failures" would read as *healthy* when nothing is
being captured at all — a misleading diagnostic is worse than none, so AC-7 was
added after implementation began.

That gap is what this spec closes. The two concurrency criteria are kept as
regression guards, honestly labelled: concurrent capture is the normal case for
this tool and nothing else asserted it worked.

## Acceptance criteria

### AC-1 Every connection waits for a held lock

THE SYSTEM SHALL set a SQLite busy timeout of at least 5000 ms on a connection
before it is used for any statement, rather than inheriting a library default.

oracle: test tests/storage.rs::busy_timeout_is_set_before_first_use

### AC-2 Concurrent terminals all persist

WHEN 16 processes each capture one command against the same database THE SYSTEM
SHALL persist 16 records.

oracle: test tests/storage.rs::sixteen_concurrent_writers_all_persist

### AC-3 A failed capture is written down

IF a capture cannot be persisted THEN THE SYSTEM SHALL append the reason and a
timestamp to a log file under the omniscient data directory.

oracle: test tests/capture.rs::a_failed_capture_is_logged

### AC-4 The log is bounded

THE SYSTEM SHALL keep the failure log at or below 1 MB by discarding its oldest
entries.

oracle: test tests/capture.rs::failure_log_is_bounded

### AC-5 A capture failure does not fail the shell

IF a capture cannot be persisted THEN THE SYSTEM SHALL exit the `capture`
subcommand with status 0, so that a storage problem never breaks the prompt.

oracle: cmd `sh tests/shell/capture_never_breaks_prompt.sh` exit 0

### AC-6 Failures are visible without reading the log by hand

WHEN `omniscient doctor` is invoked THE SYSTEM SHALL report the number of
capture failures recorded in the last 7 days.

oracle: test tests/cli.rs::doctor_reports_recent_capture_failures

### AC-7 A shell with no hook is reported, not called healthy

WHEN `omniscient doctor` is invoked and no omniscient hook is present in the
user's shell configuration THE SYSTEM SHALL report that capture is not
installed.

oracle: test tests/cli.rs::doctor_reports_a_missing_hook

## Budget note

Raised from 260 to 600 after implementation — a 2x overrun, which is a signal
about the spec rather than about the estimate. Seven criteria spanning storage,
capture, config, the CLI and shell detection, with three new integration test
files, was two specs wearing one hat: the concurrency guards and the failure
diagnostics have nothing to do with each other beyond both being about silent
loss. Splitting them would have meant two smaller reviews and an honest budget
for each.

Recorded rather than quietly absorbed, because the number was the only thing
that noticed.

## Out of scope

Retrying a failed capture, queueing captures, or changing the storage engine.
Recording every failure and waiting properly for the lock is the whole change.
