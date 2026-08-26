//! End-to-end coverage of the reporting subcommands (search, recent, top,
//! category, here, stats, export, import, config) against a real captured
//! history — no mocking of storage or capture, just the built binary.

use std::path::PathBuf;
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_omniscient"))
}

fn sandbox(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "omniscient-cli-cmd-{name}-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(dir.join(".omniscient")).unwrap();
    dir
}

/// Run a subcommand with HOME and the working directory both pinned to the
/// sandbox, so captures land in a throwaway database under a throwaway
/// working_dir.
fn run(home: &PathBuf, args: &[&str]) -> (Option<i32>, String) {
    let out = Command::new(bin())
        .args(args)
        .env("HOME", home)
        .env_remove("ZDOTDIR")
        .current_dir(home)
        .output()
        .expect("running omniscient");
    (
        out.status.code(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
    )
}

fn capture(home: &PathBuf, command: &str, exit_code: i32, duration_ms: i64) {
    let (code, out) = run(
        home,
        &[
            "capture",
            "--exit-code",
            &exit_code.to_string(),
            "--duration",
            &duration_ms.to_string(),
            command,
        ],
    );
    assert_eq!(code, Some(0), "capture of {command:?} failed: {out}");
}

#[test]
fn capture_then_stats_reports_the_command() {
    let home = sandbox("capture-stats");
    capture(&home, "git status", 0, 50);

    let (code, out) = run(&home, &["stats"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Total Commands: 1"), "{out}");
    assert!(out.contains("Successful: 1"), "{out}");
    assert!(out.contains("Commands by Category"), "{out}");
}

#[test]
fn search_finds_a_captured_command() {
    let home = sandbox("search-hit");
    capture(&home, "git commit -m test", 0, 10);
    capture(&home, "docker ps", 0, 10);

    let (code, out) = run(&home, &["search", "git"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Found 1 matching"), "{out}");
    assert!(out.contains("git commit -m test"), "{out}");
    assert!(!out.contains("docker ps"), "{out}");
}

#[test]
fn search_with_no_matches_says_so() {
    let home = sandbox("search-miss");
    capture(&home, "git status", 0, 10);

    let (code, out) = run(&home, &["search", "nonexistent"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("No commands found matching"), "{out}");
}

#[test]
fn recent_lists_captured_commands_newest_first() {
    let home = sandbox("recent");
    capture(&home, "ls -la", 0, 5);
    capture(&home, "cd /tmp", 0, 5);

    let (code, out) = run(&home, &["recent", "5"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Most recent 2 command(s)"), "{out}");
    assert!(out.contains("ls -la"), "{out}");
    assert!(out.contains("cd /tmp"), "{out}");
}

#[test]
fn recent_with_empty_history_says_so() {
    let home = sandbox("recent-empty");
    let (code, out) = run(&home, &["recent", "5"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("No commands in history yet"), "{out}");
}

#[test]
fn top_counts_repeated_usage() {
    let home = sandbox("top");
    capture(&home, "cargo build", 0, 100);
    capture(&home, "cargo build", 0, 100);

    let (code, out) = run(&home, &["top", "5"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("cargo build"), "{out}");
    assert!(out.contains("used 2 times"), "{out}");
}

#[test]
fn top_with_empty_history_says_so() {
    let home = sandbox("top-empty");
    let (code, out) = run(&home, &["top", "5"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("No commands in history yet"), "{out}");
}

#[test]
fn category_filters_to_matching_commands_only() {
    let home = sandbox("category");
    capture(&home, "git push", 0, 10);
    capture(&home, "docker ps", 0, 10);

    let (code, out) = run(&home, &["category", "git"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Commands in category 'git'"), "{out}");
    assert!(out.contains("git push"), "{out}");
    assert!(!out.contains("docker ps"), "{out}");
}

#[test]
fn category_with_no_matches_says_so() {
    let home = sandbox("category-empty");
    capture(&home, "git push", 0, 10);

    let (code, out) = run(&home, &["category", "kubernetes"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(
        out.contains("No commands found in category 'kubernetes'"),
        "{out}"
    );
}

#[test]
fn here_shows_commands_captured_in_this_directory() {
    let home = sandbox("here");
    capture(&home, "npm install", 0, 10);

    let (code, out) = run(&home, &["here"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Showing commands in"), "{out}");
    assert!(out.contains("npm install"), "{out}");
}

#[test]
fn here_with_no_commands_says_so() {
    let home = sandbox("here-empty");
    let (code, out) = run(&home, &["here"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(
        out.contains("No commands in history for this directory"),
        "{out}"
    );
}

#[test]
fn export_writes_a_json_file_with_the_captured_commands() {
    let home = sandbox("export");
    capture(&home, "git status", 0, 10);
    capture(&home, "docker ps", 0, 10);
    let export_path = home.join("out.json");

    let (code, out) = run(&home, &["export", export_path.to_str().unwrap()]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Export successful"), "{out}");
    assert!(out.contains("Commands exported: 2"), "{out}");

    let contents = std::fs::read_to_string(&export_path).unwrap();
    assert!(contents.contains("git status"));
    assert!(contents.contains("docker ps"));
}

#[test]
fn import_reads_a_previously_exported_file() {
    let source = sandbox("import-source");
    capture(&source, "git status", 0, 10);
    let export_path = source.join("out.json");
    let (code, out) = run(&source, &["export", export_path.to_str().unwrap()]);
    assert_eq!(code, Some(0), "{out}");

    let dest = sandbox("import-dest");
    let (code, out) = run(&dest, &["import", export_path.to_str().unwrap()]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Import successful"), "{out}");
    assert!(out.contains("New commands imported: 1"), "{out}");

    let (code, out) = run(&dest, &["stats"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Total Commands: 1"), "{out}");
}

#[test]
fn import_of_a_missing_file_fails_clearly() {
    let home = sandbox("import-missing");
    let (code, out) = run(&home, &["import", "does-not-exist.json"]);
    assert_ne!(code, Some(0), "{out}");
    assert!(out.contains("not found"), "{out}");
}

#[test]
fn config_prints_storage_privacy_and_capture_settings() {
    let home = sandbox("config");
    let (code, out) = run(&home, &["config"]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Storage:"), "{out}");
    assert!(out.contains("Privacy:"), "{out}");
    assert!(out.contains("Capture:"), "{out}");
}

#[test]
fn init_rejects_an_unsupported_shell() {
    let home = sandbox("init-bad-shell");
    let (code, out) = run(&home, &["init", "--shell", "fish"]);
    assert_ne!(code, Some(0), "{out}");
    assert!(out.contains("Unsupported shell"), "{out}");
}

#[test]
fn search_dir_flag_filters_to_the_named_directory() {
    let home = sandbox("search-dir");
    let other = home.join("elsewhere");
    std::fs::create_dir_all(&other).unwrap();

    capture(&home, "git status", 0, 10);
    let (code, out) = run(&home, &["search", "git", "--dir", other.to_str().unwrap()]);
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("No commands found matching"), "{out}");
}
