mod config;
mod daemon;
mod error;
mod process;

use config::Config;
use daemon::Daemon;
use error::{DaemonError, Result};
use std::env;
use std::path::PathBuf;
use std::process::exit;

fn print_usage() {
    println!("Usage: fluxion-daemon [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -C, --command <CMD>      Command to run (default: tsx)");
    println!("  -a, --args <ARGS>        Arguments for command (default: main.ts)");
    println!("  -w, --cwd <DIR>          Working directory (default: current)");
    println!("  -d, --delay <MS>         Restart delay in milliseconds (default: 1000)");
    println!("  -m, --max-restarts <N>   Maximum restarts (default: unlimited)");
    println!("  -l, --log-file <FILE>    Log file path (default: daemon.log)");
    println!("  -h, --help               Print this help");
    println!();
    println!("Examples:");
    println!("  fluxion-daemon");
    println!("  fluxion-daemon --command node --args app.js");
    println!("  fluxion-daemon --max-restarts 10 --delay 2000");
}

fn parse_args() -> Result<Config> {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                exit(0);
            }
            "-C" | "--command" => {
                if i + 1 < args.len() {
                    config.command = args[i + 1].clone();
                    i += 2;
                } else {
                    return Err(DaemonError::Config("Missing command".to_string()));
                }
            }
            "-a" | "--args" => {
                if i + 1 < args.len() {
                    config.args = args[i + 1]
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                    i += 2;
                } else {
                    return Err(DaemonError::Config("Missing arguments".to_string()));
                }
            }
            "-w" | "--cwd" => {
                if i + 1 < args.len() {
                    config.cwd = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else {
                    return Err(DaemonError::Config("Missing working directory".to_string()));
                }
            }
            "-d" | "--delay" => {
                if i + 1 < args.len() {
                    config.restart_delay = args[i + 1]
                        .parse()
                        .map_err(|_| DaemonError::Config("Invalid restart delay".to_string()))?;
                    i += 2;
                } else {
                    return Err(DaemonError::Config("Missing restart delay".to_string()));
                }
            }
            "-m" | "--max-restarts" => {
                if i + 1 < args.len() {
                    config.max_restarts = Some(
                        args[i + 1]
                            .parse()
                            .map_err(|_| DaemonError::Config("Invalid max restarts".to_string()))?
                    );
                    i += 2;
                } else {
                    return Err(DaemonError::Config("Missing max restarts".to_string()));
                }
            }
            "-l" | "--log-file" => {
                if i + 1 < args.len() {
                    config.log_file = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else {
                    return Err(DaemonError::Config("Missing log file path".to_string()));
                }
            }
            _ => {
                return Err(DaemonError::Config(format!("Unknown argument: {}", args[i])));
            }
        }
    }

    // Set default working directory to current directory
    if config.cwd == PathBuf::from(".") {
        config.cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    }

    Ok(config)
}

fn main() {
    println!("🚀 Fluxion Daemon v{}", env!("CARGO_PKG_VERSION"));
    println!("🛡️  Process Guardian for Linux Systems\n");

    // Parse command line arguments
    let config = match parse_args() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Configuration error: {}", e);
            print_usage();
            exit(1);
        }
    };

    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("❌ Configuration validation failed: {}", e);
        exit(1);
    }

    // Create and start daemon
    let mut daemon = Daemon::new(config);

    if let Err(e) = daemon.start() {
        eprintln!("❌ Failed to start daemon: {}", e);
        exit(1);
    }

    println!("✅ Daemon initialized successfully");
    println!("📝 Check daemon.log for detailed logs");
    println!("⌨️  Press Ctrl+C to stop\n");
}
