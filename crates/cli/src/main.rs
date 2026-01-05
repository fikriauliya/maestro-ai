mod config;
mod instance;
mod layout;
mod worktree;

use clap::{Parser, Subcommand};
use instance::{InstanceStore, Status};
use serde::Deserialize;
use std::io::{self, Read};
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(debug_assertions)]
const BUILD_PROFILE: &str = "debug";
#[cfg(not(debug_assertions))]
const BUILD_PROFILE: &str = "release";

fn version_string() -> &'static str {
    if cfg!(debug_assertions) {
        concat!(env!("CARGO_PKG_VERSION"), " (debug)")
    } else {
        concat!(env!("CARGO_PKG_VERSION"), " (release)")
    }
}

#[derive(Parser)]
#[command(name = "maestro")]
#[command(about = "Manage Claude Code instances in Zellij")]
#[command(version = version_string())]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print version information as JSON
    #[arg(long)]
    version_json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Register a new Claude Code instance (reads JSON from stdin)
    Register,

    /// Update status of current instance
    Update {
        /// Status to set
        #[arg(value_parser = ["running", "waiting"])]
        status: String,
    },

    /// Unregister current Claude Code instance
    Unregister,

    /// List all registered instances
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Worktree management commands
    Wt {
        #[command(subcommand)]
        command: WtCommands,
    },
}

#[derive(Subcommand)]
pub enum WtCommands {
    /// List all worktrees
    List,

    /// Switch to a worktree (creates if it doesn't exist)
    Switch {
        /// Branch name
        branch: String,
    },

    /// Remove current worktree and switch back to main
    Remove,

    /// Squash-merge current worktree to main and cleanup
    Merge,
}

#[derive(Deserialize)]
struct HookInput {
    cwd: Option<String>,
}

fn get_pane_id() -> Option<u32> {
    std::env::var("ZELLIJ_PANE_ID")
        .ok()
        .and_then(|s| s.parse().ok())
}

fn read_stdin_json() -> Option<HookInput> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).ok()?;
    serde_json::from_str(&buffer).ok()
}

fn main() {
    let cli = Cli::parse();

    if cli.version_json {
        let version_info = serde_json::json!({
            "version": VERSION,
            "build": BUILD_PROFILE
        });
        println!("{}", version_info);
        return;
    }

    let Some(command) = cli.command else {
        eprintln!("No command provided. Use --help for usage.");
        std::process::exit(1);
    };

    let store = InstanceStore::new();

    let result = match command {
        Commands::Register => {
            let pane_id = match get_pane_id() {
                Some(id) => id,
                None => {
                    eprintln!("ZELLIJ_PANE_ID not set");
                    std::process::exit(1);
                }
            };

            let input = read_stdin_json();
            let cwd = input.and_then(|i| i.cwd).unwrap_or_else(|| {
                std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default()
            });

            let folder = Path::new(&cwd)
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| cwd.clone());

            store.register(pane_id, folder)
        }

        Commands::Update { status } => {
            let pane_id = match get_pane_id() {
                Some(id) => id,
                None => {
                    eprintln!("ZELLIJ_PANE_ID not set");
                    std::process::exit(1);
                }
            };

            // Consume stdin (required by hooks)
            let _ = read_stdin_json();

            let status: Status = status.parse().unwrap();
            store.update_status(pane_id, status)
        }

        Commands::Unregister => {
            let pane_id = match get_pane_id() {
                Some(id) => id,
                None => {
                    eprintln!("ZELLIJ_PANE_ID not set");
                    std::process::exit(1);
                }
            };

            // Consume stdin (required by hooks)
            let _ = read_stdin_json();

            store.unregister(pane_id)
        }

        Commands::List { json } => {
            let instances = store.load();
            if json {
                let output = serde_json::json!({
                    "version": VERSION,
                    "build": BUILD_PROFILE,
                    "instances": instances
                });
                println!("{}", output);
            } else if instances.is_empty() {
                println!("No Claude Code instances registered");
            } else {
                for inst in instances {
                    let icon = match inst.status {
                        Status::Running => "⚡",
                        Status::Waiting => "⏳",
                    };
                    println!("{} {} (pane {})", icon, inst.folder, inst.pane_id);
                }
            }
            Ok(())
        }

        Commands::Wt { command } => {
            worktree::run(command)
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
