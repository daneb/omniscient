---
id: SPEC-0004
slug: hook-portability
schema: keel.spec/1
status: draft
scope:
  - "src/shell.rs"
  - "tests/**"
  - "CHANGELOG.md"
budget:
  criteria: 7
  lines: 240
verified_at: 2026-08-23
---

# A hook that cannot time a command must still record it

## Context

Both hooks depend on facilities they never check for, and in each case the
failure costs the command rather than the measurement.

**zsh** uses `$EPOCHREALTIME` (needs `zsh/datetime`) and `int()` (needs
`zsh/mathfunc`). Neither is loaded by the hook. Where a framework has not loaded
them, `_OMNISCIENT_START` is empty, the guard short-circuits, and **nothing is
ever captured**. Verified: `int()` under `zsh -f` fails with
`unknown function: int`.

**bash on macOS** uses `date +%s%N`. BSD `date` has no `%N`, so the timer is the
literal `1787479259N`; the arithmetic then fails with `value too great for base`,
`duration` is empty, and `capture --duration ""` cannot parse. Verified on this
machine, where `gdate` is not installed.

Both are silent. The hook discards stdout and stderr.

`product.md` settles the design question: capture fidelity outranks every other
property. A missing duration is a degraded record; a missing command is a lost
one.

## Acceptance criteria

### AC-1 The zsh hook loads what it needs

THE SYSTEM SHALL generate a zsh hook that loads `zsh/datetime` itself rather
than assuming a framework has.

oracle: cmd `cargo run --quiet -- init --shell zsh 2>/dev/null | grep -v '^[[:space:]]*#' | grep -q 'zmodload zsh/datetime'` exit 0

### AC-2 The zsh hook needs no maths module

THE SYSTEM SHALL generate a zsh hook that computes the duration without
`zsh/mathfunc`.

oracle: cmd `! cargo run --quiet -- init --shell zsh 2>/dev/null | grep -v '^[[:space:]]*#' | grep -qE 'int\(|zsh/mathfunc'` exit 0

### AC-3 A shell without the timing module still records commands

WHILE `zsh/datetime` is unavailable THE SYSTEM SHALL record the command.

oracle: cmd `zsh -f tests/shell/no_datetime.zsh` exit 0

### AC-4 The bash hook does not assume GNU date

THE SYSTEM SHALL generate a bash hook that derives its timestamp without
`date +%s%N`.

oracle: cmd `! cargo run --quiet -- init --shell bash 2>/dev/null | grep -v '^[[:space:]]*#' | grep -q 'date +%s%N'` exit 0

### AC-5 A shell without sub-second timing still records commands

WHILE no sub-second clock is available THE SYSTEM SHALL record the command with
a duration of 0.

oracle: cmd `bash tests/shell/no_subsecond_clock.sh` exit 0

### AC-6 The duration is always a whole number of milliseconds

THE SYSTEM SHALL pass `capture --duration` a non-negative integer.

oracle: cmd `zsh -f tests/shell/duration_is_integer.zsh` exit 0

### AC-7 Existing installs are told to re-hook

THE SYSTEM SHALL record in `CHANGELOG.md` that this fix reaches an existing
installation only after `omniscient init` is re-run.

oracle: cmd `grep -q 'zsh/datetime' CHANGELOG.md` exit 0

## A note on these oracles

Three of them grep the generated hook while excluding comment lines. Grepping
the whole artefact does not work: a criterion of the form "the hook must not use
X" is naturally implemented alongside a comment saying *why* it does not use X,
and that comment matches. The criterion is about what the hook does, so the
oracle reads code and not prose.

## Out of scope

Adding a dependency on `gdate` or coreutils. Changing what a duration means, or
storing durations differently. Fish.
