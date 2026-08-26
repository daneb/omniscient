//! Storage behaviour under the conditions omniscient actually runs in:
//! several terminals, all writing one SQLite file at once.

use omniscient::{CommandRecord, Storage};
use std::sync::{Arc, Barrier};

fn tmp_db(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "omniscient-it-{name}-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("history.db")
}

fn record(cmd: &str) -> CommandRecord {
    CommandRecord::new(
        cmd.to_string(),
        chrono::Utc::now(),
        0,
        1,
        String::from("/tmp"),
        String::from("other"),
    )
}

/// AC-1 — a regression guard, not a bug fix.
///
/// rusqlite defaults this to 5s, so it already held. The test exists so that a
/// future change to how connections are opened cannot quietly remove it.
#[test]
fn busy_timeout_is_set_before_first_use() {
    let db = tmp_db("timeout");
    let storage = Storage::new(&db).expect("storage");

    let timeout = storage.busy_timeout_ms().expect("busy_timeout");
    assert!(
        timeout >= 5000,
        "busy_timeout is {timeout}ms; a writer that does not wait loses the command"
    );
}

/// AC-2 — a regression guard for concurrent capture.
///
/// This was written expecting it to reproduce lost writes across terminals. It
/// does not: it passes with the busy timeout removed, because rusqlite's default
/// already covers the contention. Kept because concurrent capture is the normal
/// case for this tool and nothing else asserts it works.
#[test]
fn sixteen_concurrent_writers_all_persist() {
    let db = tmp_db("concurrent");
    // Create the schema once up front, as the first terminal of the day would.
    Storage::new(&db).expect("initial storage");

    const WRITERS: usize = 16;
    let barrier = Arc::new(Barrier::new(WRITERS));
    let mut handles = Vec::new();

    for i in 0..WRITERS {
        let db = db.clone();
        let barrier = Arc::clone(&barrier);
        handles.push(std::thread::spawn(move || {
            let storage = Storage::new(&db).expect("storage");
            // Everybody hits the lock at the same instant.
            barrier.wait();
            storage.insert(&record(&format!("command-{i}")))
        }));
    }

    let mut failures = Vec::new();
    for (i, h) in handles.into_iter().enumerate() {
        match h.join().expect("thread") {
            Ok(_) => {}
            Err(e) => failures.push(format!("writer {i}: {e}")),
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {WRITERS} concurrent captures were lost:\n  {}",
        failures.len(),
        failures.join("\n  ")
    );

    let storage = Storage::new(&db).expect("storage");
    let stats = storage.get_stats().expect("stats");
    assert_eq!(
        stats.total_commands, WRITERS,
        "expected {WRITERS} commands on disk, found {}",
        stats.total_commands
    );
}
