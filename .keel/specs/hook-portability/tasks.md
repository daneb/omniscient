---
id: TASKS-0004
slug: hook-portability
schema: keel.tasks/1
---

# Tasks

### T-1 Tests for a shell without the facilities
- criteria: AC-3, AC-5, AC-6
- files: tests/shell/no_datetime.zsh, tests/shell/no_subsecond_clock.sh, tests/shell/duration_is_integer.zsh
- budget: 90
- exit: all three fail against the current hook

### T-2 Make timing best-effort in both hooks
- criteria: AC-1, AC-2, AC-4
- files: src/shell.rs
- budget: 100
- exit: the generated hooks contain no int(), no mathfunc and no `date +%s%N`
- depends_on: T-1

### T-3 Tell existing installs
- criteria: AC-7
- files: CHANGELOG.md
- budget: 30
- exit: the changelog names zsh/datetime and the re-init
- depends_on: T-2
