/// Main CLI entry point for Omniscient
use clap::{Parser, Subcommand};
use colored::Colorize;
use omniscient::{Config, Result};
use std::env;

#[derive(Parser)]
#[command(name = "omniscient")]
#[command(about = "CLI command history tracker - never forget a command again", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize shell integration (generates hook code)
    Init {
        /// Specify shell type (zsh, bash). Auto-detected if not provided.
        #[arg(long)]
        shell: Option<String>,
    },

    /// Report whether omniscient is actually recording
    Doctor,

    /// Capture a command (internal use by shell hook)
    Capture {
        /// Exit code of the command
        #[arg(long)]
        exit_code: i32,

        /// Duration in milliseconds
        #[arg(long)]
        duration: i64,

        /// The command to capture
        command: String,
    },

    /// Search command history
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,

        /// Filter by directory
        #[arg(short, long)]
        dir: Option<String>,

        /// Include subdirectories
        #[arg(short, long)]
        recursive: bool,
    },

    /// Show commands executed in current directory
    Here {
        /// Include commands from subdirectories
        #[arg(short, long)]
        recursive: bool,

        /// Directory to query (default: current directory)
        #[arg(short, long)]
        dir: Option<String>,

        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// Show recent commands
    Recent {
        /// Number of commands to show
        #[arg(default_value = "20")]
        n: usize,

        /// Filter by directory
        #[arg(short, long)]
        dir: Option<String>,

        /// Include subdirectories
        #[arg(short, long)]
        recursive: bool,
    },

    /// Show most frequently used commands
    Top {
        /// Number of commands to show
        #[arg(default_value = "10")]
        n: usize,

        /// Filter by directory
        #[arg(short, long)]
        dir: Option<String>,

        /// Include subdirectories
        #[arg(short, long)]
        recursive: bool,
    },

    /// Filter commands by category
    Category {
        /// Category name (git, docker, etc.)
        name: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,

        /// Filter by directory
        #[arg(short, long)]
        dir: Option<String>,

        /// Include subdirectories
        #[arg(short, long)]
        recursive: bool,
    },

    /// Show usage statistics
    Stats,

    /// Export command history to JSON
    Export {
        /// Output file path
        #[arg(default_value = "history.json")]
        file: String,
    },

    /// Import command history from JSON
    Import {
        /// Input file path
        file: String,
    },

    /// Show configuration
    Config,
}

/// Return a colored status symbol for a command record
fn colorize_status(cmd: &omniscient::CommandRecord) -> colored::ColoredString {
    if cmd.is_success() {
        "✓".green()
    } else {
        "✗".red()
    }
}

/// Return a colored string for a category name
fn colorize_category(category: &str) -> colored::ColoredString {
    match category {
        "git" => category.cyan(),
        "docker" => category.blue(),
        "network" => category.magenta(),
        "file" => category.yellow(),
        "package" => category.bright_green(),
        "database" => category.bright_magenta(),
        "kubernetes" => category.bright_blue(),
        "cloud" => category.bright_cyan(),
        "system" => category.bright_yellow(),
        "editor" => category.white(),
        "build" => category.bright_red(),
        "vcs" => category.bright_white(),
        _ => category.normal(),
    }
}

/// Highlight the first occurrence of `query` in `text` using bold + underline
fn highlight_match(text: &str, query: &str) -> String {
    let lower_text = text.to_lowercase();
    let lower_query = query.to_lowercase();
    match lower_text.find(&lower_query) {
        Some(start) => {
            let end = start + query.len();
            let prefix = &text[..start];
            let matched = &text[start..end];
            let suffix = &text[end..];
            format!("{}{}{}", prefix, matched.bold().underline(), suffix)
        }
        None => text.to_string(),
    }
}

/// Resolve the directory to query (from --dir flag or current directory)
fn resolve_directory(dir_arg: Option<String>) -> Result<String> {
    match dir_arg {
        Some(path) => Ok(path),
        None => env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .map_err(omniscient::OmniscientError::Io),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration.
    //
    // `capture` runs on every prompt and must never change the exit status,
    // which means setup failing is not allowed to be fatal either. A home
    // directory that cannot be read, or an .omniscient path occupied by
    // something else, would otherwise turn a broken install into a broken shell.
    let config = match Config::load().and_then(|c| c.ensure_directories().map(|_| c)) {
        Ok(c) => c,
        Err(e) => {
            if let Commands::Capture { command, .. } = &cli.command {
                // Best effort: without a config the log path is a guess, and
                // logging is infallible by design.
                omniscient::capture::log_capture_failure(
                    &Config::default(),
                    command,
                    &format!("could not start: {e}"),
                );
                eprintln!("omniscient: capture error: could not start: {e}");
                return Ok(());
            }
            return Err(e);
        }
    };

    match cli.command {
        Commands::Init { shell } => {
            use omniscient::ShellType;

            // Determine shell type (manual or auto-detect)
            let shell_type = if let Some(shell_name) = shell {
                match shell_name.as_str() {
                    "zsh" => ShellType::Zsh,
                    "bash" => ShellType::Bash,
                    _ => {
                        eprintln!(
                            "Error: Unsupported shell '{}'. Supported shells: zsh, bash",
                            shell_name
                        );
                        eprintln!("Tip: Omit --shell flag to auto-detect your shell.");
                        std::process::exit(1);
                    }
                }
            } else {
                omniscient::ShellHook::detect_shell()?
            };

            let hook = omniscient::ShellHook::new(shell_type);
            println!("{}", hook.generate());
            eprintln!("{}", hook.installation_instructions());
            Ok(())
        }
        Commands::Doctor => {
            let db = config.database_path()?;
            println!("database   {}", db.display());
            match omniscient::Storage::new(&db) {
                Ok(storage) => {
                    let stats = storage.get_stats()?;
                    println!("commands   {} recorded", stats.total_commands);
                }
                Err(e) => println!("database   UNREADABLE — {e}"),
            }

            // The most complete failure is the one that never errors: no hook
            // installed, so nothing is captured and nothing is logged.
            let rc_files = omniscient::ShellHook::candidate_rc_files();
            match omniscient::ShellHook::installed_in(&rc_files) {
                Some(f) => println!("hook       installed in {}", f.display()),
                None => println!(
                    "hook       NOT INSTALLED — nothing is being captured.\n\
                     \x20          Run `omniscient init` and add the output to your shell rc file."
                ),
            }

            // The number that answers "is this thing actually working?". The
            // shell hook discards output, so without this a failing capture is
            // only discoverable by noticing an absence months later.
            let failures = omniscient::capture::recent_failures(&config, 7);
            if failures == 0 {
                println!("failures   none in the last 7 days");
            } else {
                println!(
                    "failures   {failures} capture(s) failed in the last 7 days — see {}",
                    config.failure_log_path()?.display()
                );
            }
            Ok(())
        }

        Commands::Capture {
            exit_code,
            duration,
            command,
        } => {
            // The shell must keep working, so a capture failure never changes
            // the exit status. But it must not vanish either: the hook discards
            // both streams, so stderr alone means nobody ever learns.
            //
            // Both paths can fail. Opening storage is the likelier one — a
            // locked or unreadable database — and propagating that with `?`
            // would have exited non-zero from inside the prompt.
            let log_config = config.clone();
            let note = |reason: String| {
                omniscient::capture::log_capture_failure(&log_config, &command, &reason);
                eprintln!("omniscient: capture error: {reason}");
            };

            match omniscient::CommandCapture::new(config) {
                Ok(capture) => {
                    if let Err(e) = capture.capture(&command, exit_code, duration) {
                        note(e.to_string());
                    }
                }
                Err(e) => note(format!("could not open storage: {e}")),
            }

            Ok(())
        }
        Commands::Search {
            query,
            limit,
            dir,
            recursive,
        } => {
            let storage = omniscient::Storage::new(&config.database_path()?)?;

            let working_dir = if dir.is_some() {
                Some(resolve_directory(dir)?)
            } else {
                None
            };

            let search_query = omniscient::SearchQuery {
                text: Some(query.clone()),
                category: None,
                success_only: None,
                working_dir,
                recursive,
                limit,
                order_by: omniscient::OrderBy::Relevance,
            };

            let results = storage.search(&search_query)?;

            if results.is_empty() {
                println!("No commands found matching '{}'", query);
                return Ok(());
            }

            println!("\nFound {} matching command(s):\n", results.len());
            for cmd in results {
                println!(
                    "[{}] {} {}",
                    cmd.timestamp
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                        .dimmed(),
                    colorize_status(&cmd),
                    highlight_match(&cmd.command, &query)
                );
                println!(
                    "  Category: {} | Duration: {} | Usage: {} times | Dir: {}",
                    colorize_category(&cmd.category),
                    cmd.duration_display(),
                    cmd.usage_count,
                    cmd.working_dir.dimmed()
                );
                println!();
            }

            Ok(())
        }
        Commands::Here {
            recursive,
            dir,
            limit,
        } => {
            let storage = omniscient::Storage::new(&config.database_path()?)?;
            let working_dir = Some(resolve_directory(dir)?);

            let results = storage.get_recent(limit, working_dir.clone(), recursive)?;

            if results.is_empty() {
                println!("No commands in history for this directory.");
                return Ok(());
            }

            // Display header with context
            let dir_display = working_dir.as_ref().unwrap();
            let mode = if recursive {
                "(recursive)"
            } else {
                "(exact match)"
            };
            println!("\nShowing commands in: {} {}\n", dir_display, mode);
            println!("Found {} command(s):\n", results.len());

            // Reuse display format from Recent command
            for cmd in results {
                println!(
                    "[{}] {} {}",
                    cmd.timestamp
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                        .dimmed(),
                    colorize_status(&cmd),
                    cmd.command
                );
                println!(
                    "  Dir: {} | Category: {} | Duration: {} | Usage: {} times",
                    cmd.working_dir.dimmed(),
                    colorize_category(&cmd.category),
                    cmd.duration_display(),
                    cmd.usage_count
                );
                println!();
            }

            Ok(())
        }
        Commands::Recent { n, dir, recursive } => {
            let storage = omniscient::Storage::new(&config.database_path()?)?;

            let working_dir = if dir.is_some() {
                Some(resolve_directory(dir)?)
            } else {
                None
            };

            let results = storage.get_recent(n, working_dir, recursive)?;

            if results.is_empty() {
                println!("No commands in history yet.");
                return Ok(());
            }

            println!("\nMost recent {} command(s):\n", results.len());
            for cmd in results {
                println!(
                    "[{}] {} {}",
                    cmd.timestamp
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                        .dimmed(),
                    colorize_status(&cmd),
                    cmd.command
                );
                println!(
                    "  Category: {} | Duration: {} | Usage: {} times",
                    colorize_category(&cmd.category),
                    cmd.duration_display(),
                    cmd.usage_count
                );
                println!();
            }

            Ok(())
        }
        Commands::Top { n, dir, recursive } => {
            let storage = omniscient::Storage::new(&config.database_path()?)?;

            let working_dir = if dir.is_some() {
                Some(resolve_directory(dir)?)
            } else {
                None
            };

            let results = storage.get_top(n, working_dir, recursive)?;

            if results.is_empty() {
                println!("No commands in history yet.");
                return Ok(());
            }

            println!("\nTop {} most frequently used command(s):\n", results.len());
            for (index, cmd) in results.iter().enumerate() {
                println!(
                    "{}. {} (used {} times)",
                    index + 1,
                    cmd.command,
                    cmd.usage_count.to_string().bold()
                );
                println!(
                    "   Category: {} | Last used: {} | Avg duration: {}",
                    colorize_category(&cmd.category),
                    cmd.last_used
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                        .dimmed(),
                    cmd.duration_display()
                );
                println!();
            }

            Ok(())
        }
        Commands::Category {
            name,
            limit,
            dir,
            recursive,
        } => {
            let storage = omniscient::Storage::new(&config.database_path()?)?;

            let working_dir = if dir.is_some() {
                Some(resolve_directory(dir)?)
            } else {
                None
            };

            let results = storage.get_by_category(&name, limit, working_dir, recursive)?;

            if results.is_empty() {
                println!("No commands found in category '{}'", name);
                return Ok(());
            }

            println!(
                "\nCommands in category '{}' ({} found):\n",
                name,
                results.len()
            );
            for cmd in results {
                println!(
                    "[{}] {} {}",
                    cmd.last_used
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                        .dimmed(),
                    colorize_status(&cmd),
                    cmd.command
                );
                println!(
                    "  Used {} times | Duration: {} | Dir: {}",
                    cmd.usage_count.to_string().bold(),
                    cmd.duration_display(),
                    cmd.working_dir.dimmed()
                );
                println!();
            }

            Ok(())
        }
        Commands::Stats => {
            let storage = omniscient::Storage::new(&config.database_path()?)?;
            let stats = storage.get_stats()?;

            println!("\n=== Omniscient Command History Statistics ===\n");

            println!("Total Commands: {}", stats.total_commands);
            println!(
                "Successful: {} ({:.1}%)",
                stats.successful_commands,
                stats.success_rate()
            );
            println!(
                "Failed: {} ({:.1}%)",
                stats.failed_commands,
                100.0 - stats.success_rate()
            );

            if let (Some(oldest), Some(newest)) = (&stats.oldest_command, &stats.newest_command) {
                println!("\nTime Range:");
                println!("  First command: {}", oldest.format("%Y-%m-%d %H:%M:%S"));
                println!("  Last command:  {}", newest.format("%Y-%m-%d %H:%M:%S"));

                let duration = *newest - *oldest;
                let days = duration.num_days();
                if days > 0 {
                    println!("  Tracking for:  {} days", days);
                    println!(
                        "  Avg per day:   {:.1} commands",
                        stats.total_commands as f64 / days as f64
                    );
                }
            }

            if !stats.by_category.is_empty() {
                println!("\nCommands by Category:");
                for cat_stat in &stats.by_category {
                    let percentage = (cat_stat.count as f64 / stats.total_commands as f64) * 100.0;
                    println!(
                        "  {:12} {:5} ({:.1}%)",
                        colorize_category(&cat_stat.category),
                        cat_stat.count,
                        percentage
                    );
                }
            }

            println!();
            Ok(())
        }
        Commands::Export { file } => {
            let storage = omniscient::Storage::new(&config.database_path()?)?;
            let exporter = omniscient::Exporter::new(storage);

            println!("Exporting command history to {}...", file);

            match exporter.export(&file) {
                Ok(stats) => {
                    println!("\n✓ Export successful!");
                    println!("  Commands exported: {}", stats.commands_exported);
                    println!("  File: {}", stats.file_path);
                    println!("\nYou can now:");
                    println!("  - Backup this file to version control");
                    println!("  - Import it on another machine");
                    println!("  - Share it with your team");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("✗ Export failed: {}", e);
                    Err(e)
                }
            }
        }
        Commands::Import { file } => {
            let storage = omniscient::Storage::new(&config.database_path()?)?;

            // Check if file exists
            if !std::path::Path::new(&file).exists() {
                eprintln!("✗ Error: File '{}' not found", file);
                return Err(omniscient::OmniscientError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("File '{}' not found", file),
                )));
            }

            println!("Importing command history from {}...", file);

            // Use PreserveHigher strategy by default (keeps the higher usage count)
            let importer =
                omniscient::Importer::new(storage, omniscient::ImportStrategy::PreserveHigher);

            match importer.import(&file) {
                Ok(stats) => {
                    println!("\n✓ Import successful!");
                    println!("  Total commands in file: {}", stats.total_commands);
                    println!("  New commands imported: {}", stats.imported);
                    println!("  Existing commands updated: {}", stats.updated);
                    println!("  Duplicates skipped: {}", stats.skipped);
                    println!("\n{}", stats.summary());
                    Ok(())
                }
                Err(e) => {
                    eprintln!("✗ Import failed: {}", e);
                    Err(e)
                }
            }
        }
        Commands::Config => {
            println!("Configuration:");
            println!(
                "  Storage: {} at {}",
                config.storage.storage_type, config.storage.path
            );
            println!(
                "  Privacy: {} (patterns: {})",
                if config.privacy.enabled {
                    "enabled"
                } else {
                    "disabled"
                },
                config.privacy.redact_patterns.len()
            );
            println!(
                "  Capture: min_duration={}ms, max_history={}",
                config.capture.min_duration_ms, config.capture.max_history_size
            );
            Ok(())
        }
    }
}
