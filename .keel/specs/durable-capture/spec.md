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
  - "tests/**"
budget:
  criteria: 6
  lines: 180
verified_at: 2026-08-23
---

# A capture that fails must not fail silently

## Context

Every terminal is a separate process writing one SQLite file. `storage.rs` sets
`journal_mode=WAL` but never sets a busy timeout, so a connection that finds the
write lock held returns `SQLITE_BUSY` **immediately** rather than waiting. WAL
permits one writer at a time; with several terminals at a prompt together, some
writes lose the race.

The loss is silent because the shell hook runs capture as
`omniscient capture ... &>/dev/null &!`. Both streams are discarded, so the
error reaches nobody. The two faults compound: without the timeout there are
failures, and without a log there is no evidence they happened.

## Acceptance criteria

### AC-1 Every connection waits for a held lock

THE SYSTEM SHALL set a SQLite busy timeout of at least 5000 ms on a connection
before it is used for any statement.

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
subcommand with status 0.

oracle: cmd `sh tests/shell/capture_never_breaks_prompt.sh` exit 0

### AC-6 Failures are visible without reading the log by hand

WHEN `omniscient doctor` is invoked THE SYSTEM SHALL report the number of
capture failures recorded in the last 7 days.

oracle: test tests/cli.rs::doctor_reports_recent_capture_failures

## Out of scope

Retrying a failed capture, queueing captures, or changing the storage engine.
Recording every failure and waiting properly for the lock is the whole change.
