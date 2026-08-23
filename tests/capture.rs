//! A capture that fails must leave a trace.
//!
//! The shell hook runs `omniscient capture ... &>/dev/null &!`, so both output
//! streams are discarded. Anything that only prints is invisible, and the only
//! symptom of a broken capture is an absence noticed months later.

use omniscient::capture::{log_capture_failure, recent_failures};
use omniscient::Config;

fn sandbox(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "omniscient-cap-{name}-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn config_in(dir: &std::path::Path) -> Config {
    let mut config = Config::default();
    config.storage.path = dir.join("history.db").to_string_lossy().to_string();
    config
}

/// AC-3 — a failed capture is written down, with a reason and a timestamp.
#[test]
fn a_failed_capture_is_logged() {
    let dir = sandbox("logged");
    let config = config_in(&dir);

    log_capture_failure(&config, "git push --force", "database is locked");

    let log = config.failure_log_path().expect("log path");
    let contents = std::fs::read_to_string(&log).expect("the failure was not recorded anywhere");

    assert!(
        contents.contains("database is locked"),
        "the reason is missing from: {contents}"
    );
    assert!(
        contents.contains("git push --force"),
        "the command is missing from: {contents}"
    );
    // A timestamp, so "when did this start?" is answerable.
    let stamp = contents.split('\t').next().unwrap_or_default();
    assert!(
        chrono::DateTime::parse_from_rfc3339(stamp).is_ok(),
        "no RFC 3339 timestamp at the start of: {contents}"
    );
}

/// AC-4 — the log cannot grow without bound.
///
/// A machine where capture is persistently broken would otherwise turn one
/// problem into two.
#[test]
fn failure_log_is_bounded() {
    let dir = sandbox("bounded");
    let config = config_in(&dir);
    let log = config.failure_log_path().expect("log path");

    // Comfortably past the 1 MB ceiling.
    for i in 0..20_000 {
        log_capture_failure(&config, &format!("command number {i} with some padding"), "boom");
    }

    let size = std::fs::metadata(&log).expect("log").len();
    assert!(
        size <= 1024 * 1024,
        "failure log grew to {size} bytes, past the 1 MB ceiling"
    );

    // Trimming keeps the newest entries, which are the useful ones.
    let contents = std::fs::read_to_string(&log).unwrap();
    assert!(
        contents.contains("command number 19999"),
        "the most recent failure was trimmed away"
    );
}

/// AC-6's counting half — failures inside the window are counted, older ones not.
#[test]
fn recent_failures_counts_only_the_window() {
    let dir = sandbox("window");
    let config = config_in(&dir);
    let log = config.failure_log_path().expect("log path");

    let old = (chrono::Utc::now() - chrono::Duration::days(30)).to_rfc3339();
    let new = chrono::Utc::now().to_rfc3339();
    std::fs::write(
        &log,
        format!("{old}\tboom\tancient\n{new}\tboom\trecent\n{new}\tboom\talso recent\n"),
    )
    .unwrap();

    assert_eq!(recent_failures(&config, 7), 2, "the window is not being applied");
    assert_eq!(recent_failures(&config, 365), 3);
}
