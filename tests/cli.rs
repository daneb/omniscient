//! `omniscient doctor` — can a user find out whether this tool is working?

use std::path::PathBuf;
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_omniscient"))
}

fn sandbox(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "omniscient-cli-{name}-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(dir.join(".omniscient")).unwrap();
    dir
}

/// Run doctor with HOME pointed at a sandbox, so the real database and the
/// real shell configuration are never touched.
fn doctor_in(home: &PathBuf) -> String {
    let out = Command::new(bin())
        .arg("doctor")
        .env("HOME", home)
        .env_remove("ZDOTDIR")
        .output()
        .expect("running doctor");
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// AC-6 — failures are reachable without knowing the log file exists.
#[test]
fn doctor_reports_recent_capture_failures() {
    let home = sandbox("failures");
    let log = home.join(".omniscient/capture-failures.log");
    let now = chrono::Utc::now().to_rfc3339();
    std::fs::write(&log, format!("{now}\tdatabase is locked\tgit status\n")).unwrap();

    let out = doctor_in(&home);
    assert!(
        out.contains("1 capture"),
        "doctor did not report the failure:\n{out}"
    );
    assert!(
        out.contains("7 days"),
        "doctor did not say over what window:\n{out}"
    );
}

/// AC-7 — the most complete failure is the one that never errors.
///
/// With no hook installed nothing is captured, nothing fails and nothing is
/// logged. A doctor that reported "no failures" here would read as healthy.
#[test]
fn doctor_reports_a_missing_hook() {
    let home = sandbox("nohook");
    std::fs::write(home.join(".zshrc"), "# nothing to see here\n").unwrap();

    let out = doctor_in(&home);
    assert!(
        out.contains("NOT INSTALLED"),
        "doctor called a shell with no hook healthy:\n{out}"
    );
    assert!(
        out.contains("omniscient init"),
        "doctor did not say how to fix it:\n{out}"
    );
}

/// The other half: a hook that is installed is recognised.
#[test]
fn doctor_recognises_an_installed_hook() {
    let home = sandbox("hooked");
    std::fs::write(
        home.join(".zshrc"),
        "precmd_functions+=(_omniscient_precmd)\n",
    )
    .unwrap();

    let out = doctor_in(&home);
    assert!(
        out.contains("installed in"),
        "doctor did not recognise an installed hook:\n{out}"
    );
    assert!(!out.contains("NOT INSTALLED"), "{out}");
}
