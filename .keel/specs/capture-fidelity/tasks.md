---
id: TASKS-0001
slug: capture-fidelity
schema: keel.tasks/1
---

# Tasks

### T-1 Shell test harness
- criteria: AC-5
- files: tests/shell/bare_prompt.zsh
- budget: 40
- exit: `zsh -f tests/shell/bare_prompt.zsh` exits 0 against the current hook

### T-2 Take the command from preexec in both hooks
- criteria: AC-1, AC-6
- files: src/shell.rs
- budget: 60
- exit: neither `fc -ln` nor `history 1` appears in src/shell.rs

### T-4 Tell existing installs to re-hook
- criteria: AC-7
- files: CHANGELOG.md
- budget: 30
- exit: CHANGELOG names `omniscient init` as the required action
- depends_on: T-2

### T-3 Prove it under each history option
- criteria: AC-2, AC-3, AC-4
- files: tests/shell/share_history.zsh, tests/shell/ignore_space.zsh, tests/shell/ignore_dups.zsh
- budget: 70
- exit: all three scripts exit 0
- depends_on: T-2
