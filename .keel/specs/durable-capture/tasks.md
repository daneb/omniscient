---
id: TASKS-0002
slug: durable-capture
schema: keel.tasks/1
---

# Tasks

### T-1 Busy timeout on every connection
- criteria: AC-1, AC-2
- files: src/storage.rs, tests/storage.rs
- budget: 60
- exit: 16 concurrent writers all persist

### T-2 A bounded failure log
- criteria: AC-3, AC-4, AC-5
- files: src/capture.rs, src/config.rs, tests/capture.rs
- budget: 80
- exit: a forced failure appears in the log, the log stays under 1 MB, and capture exits 0

### T-3 Surface the failures in doctor
- criteria: AC-6
- files: src/main.rs, tests/cli.rs
- budget: 40
- exit: `omniscient doctor` prints the 7-day failure count
- depends_on: T-2
