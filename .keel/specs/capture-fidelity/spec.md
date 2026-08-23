---
id: SPEC-0001
slug: capture-fidelity
schema: keel.spec/1
status: draft
scope:
  - "src/shell.rs"
  - "tests/**"
budget:
  criteria: 6
  lines: 180
verified_at: 2026-08-23
---

# Capture the command the shell ran, not the one in history

## Context

The zsh hook reads the command in `precmd` with `fc -ln -1`, which asks the
shell's *history* what happened. History is not a record of this session:

- `SHARE_HISTORY` (set on this machine) imports other sessions' commands before
  each prompt, so `fc -ln -1` can return a command typed in another terminal.
- `HIST_IGNORE_SPACE` (set) means a space-prefixed command never enters history,
  so `fc -ln -1` returns the *previous* command and it is captured twice while
  the real one is lost.
- `HIST_IGNORE_DUPS` (set) suppresses repeats, so a repeated command reports a
  stale timestamp.

`preexec` is already invoked with the command as `$1` and currently ignores it.
That argument is authoritative and unaffected by every option above.

## Acceptance criteria

### AC-1 The command comes from preexec, not from history

THE SYSTEM SHALL generate a zsh hook whose `precmd` records the command text
captured by `preexec` from its first argument, and whose `precmd` does not
invoke `fc`.

oracle: cmd `! cargo run --quiet -- init --shell zsh 2>/dev/null | grep -q 'fc -ln'` exit 0

### AC-2 Shared history does not misattribute a command

WHILE `SHARE_HISTORY` is set THE SYSTEM SHALL record the command entered in the
current shell.

oracle: cmd `zsh -f tests/shell/share_history.zsh` exit 0

### AC-3 A space-prefixed command does not displace its predecessor

WHILE `HIST_IGNORE_SPACE` is set THE SYSTEM SHALL NOT record the previous
command a second time when a space-prefixed command is run.

oracle: cmd `zsh -f tests/shell/ignore_space.zsh` exit 0

### AC-4 A repeated command is recorded each time it is run

WHILE `HIST_IGNORE_DUPS` is set THE SYSTEM SHALL record two entries when the
same command is run twice.

oracle: cmd `zsh -f tests/shell/ignore_dups.zsh` exit 0

### AC-5 A prompt with no preceding command records nothing

IF `precmd` runs without a preceding `preexec` THEN THE SYSTEM SHALL record
nothing.

oracle: cmd `zsh -f tests/shell/bare_prompt.zsh` exit 0

### AC-6 The bash hook uses the same source

THE SYSTEM SHALL generate a bash hook that records the command bash-preexec
passed to `preexec`, and that does not invoke the `history` builtin.

oracle: cmd `! cargo run --quiet -- init --shell bash 2>/dev/null | grep -q 'history 1'` exit 0

## Out of scope

Fish support. Migrating history already captured under the old hook. Changing
what is stored — only where the command text comes from.
