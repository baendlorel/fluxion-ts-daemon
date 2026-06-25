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
    println!("  -c, --command <CMD>      Full command to run (default: tsx main.ts)");
    println!("  -h, --help               Print this help");
    println!();
    println!("Examples:");
    println!("  fluxion-daemon");
    println!("  fluxion-daemon -c \"node app.js\"");
    println!("  fluxion-daemon -c \"python server.py --port 3000\"");
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
            "-c" | "--command" => {
                if i + 1 < args.len() {
                    let full_cmd = &args[i + 1];
                    let parts: Vec<&str> = full_cmd.split_whitespace().collect();
                    if !parts.is_empty() {
                        config.command = parts[0].to_string();
                        config.args = parts[1..].iter().map(|s| s.to_string()).collect();
                    }
                    i += 2;
                } else {
                    return Err(DaemonError::Config("Missing command".to_string()));
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
