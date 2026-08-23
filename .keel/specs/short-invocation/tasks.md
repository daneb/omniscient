---
id: TASKS-0003
slug: short-invocation
schema: keel.tasks/1
---

# Tasks

### T-1 A second binary target
- criteria: AC-1, AC-2, AC-5
- files: Cargo.toml, src/main.rs, tests/cli.rs
- budget: 50
- exit: `omni` and `omniscient` are built and behave identically

### T-2 Install and uninstall both
- criteria: AC-3, AC-4
- files: install.sh, uninstall.sh, tests/install/both_names_installed.sh, tests/install/both_names_removed.sh
- budget: 60
- exit: both names arrive on PATH and both are removed
- depends_on: T-1
