---
id: TECH-0001
scope: repo
owner: human
verified_at: 2026-08-23
---

# Tech

## Stack

- Rust (Cargo), edition 2021, single binary
- SQLite via rusqlite, WAL journal
- Shell integration: zsh `preexec`/`precmd`; bash via bash-preexec

## Build and test

```sh
# build: cargo build --release
# test:  cargo test
# lint:  cargo clippy --all-targets
```

## Constraints

- **Capture must not block the prompt.** The hook runs on every command; the
  budget is single-digit milliseconds of shell-visible latency.
- **The hook discards output.** It redirects to /dev/null and backgrounds, so
  anything written to stdout or stderr is invisible. A failure that only prints
  is a failure that never happened.
- **Many writers, one database.** Every terminal is a separate process writing
  the same SQLite file, concurrently.
- Shell hook code is shipped as a string from `src/shell.rs` and pasted into a
  user's rc file. Changing it does not change already-installed hooks.
