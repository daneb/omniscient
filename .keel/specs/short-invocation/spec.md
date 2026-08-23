---
id: SPEC-0003
slug: short-invocation
schema: keel.spec/1
status: draft
scope:
  - "Cargo.toml"
  - "src/main.rs"
  - "install.sh"
  - "uninstall.sh"
  - "tests/**"
budget:
  criteria: 5
  lines: 150
verified_at: 2026-08-23
---

# A name you can type

## Context

The binary is `omniscient`: ten characters, awkward to type, and typed several
times a day. There is no `[[bin]]` section, so the binary name is simply the
crate name.

A shorter name is not cosmetic here. A tool whose whole value is recall gets
used in proportion to how cheap it is to reach for, and `omniscient search foo`
is expensive enough that a scrollback search wins.

`omni` is four characters, unambiguous, and unclaimed on this machine.

## Acceptance criteria

### AC-1 A short binary is built

THE SYSTEM SHALL build a binary named `omni` alongside `omniscient`.

oracle: cmd `cargo build --quiet && test -x target/debug/omni` exit 0

### AC-2 The two names behave identically

WHEN the same arguments are given to `omni` and to `omniscient` THE SYSTEM
SHALL produce the same output and the same exit status.

oracle: test tests/cli.rs::omni_and_omniscient_are_equivalent

### AC-3 Installing provides both

WHEN `install.sh` completes THE SYSTEM SHALL place both `omni` and
`omniscient` on the user's PATH.

oracle: cmd `sh tests/install/both_names_installed.sh` exit 0

### AC-4 Uninstalling removes both

WHEN `uninstall.sh` completes THE SYSTEM SHALL remove both names.

oracle: cmd `sh tests/install/both_names_removed.sh` exit 0

### AC-5 The shell hook does not depend on the short name

THE SYSTEM SHALL generate hook code that invokes `omniscient`, so that an
install without the short name keeps capturing.

oracle: cmd `grep -q 'omniscient capture' src/shell.rs` exit 0

## Out of scope

Renaming the crate, the repository or the published package. Shell completions.
Any change to subcommand names — `omni search` is short enough.
