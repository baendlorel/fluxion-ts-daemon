use crate::config::Config;
use crate::error::{DaemonError, Result};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

pub struct ProcessManager {
    config: Config,
    child_process: Option<Child>,
    restart_count: u32,
}

impl ProcessManager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            child_process: None,
            restart_count: 0,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.log("Starting process manager...", "INFO");

        loop {
            if let Some(max_restarts) = self.config.max_restarts {
                if self.restart_count >= max_restarts {
                    let msg = format!(
                        "Maximum restart limit ({}) reached. Stopping daemon.",
                        max_restarts
                    );
                    self.log(&msg, "ERROR");
                    return Err(DaemonError::Process(msg));
                }
            }

            self.spawn_process()?;
            self.wait_for_exit()?;

            self.log(&format!("Scheduling restart #{}", self.restart_count + 1), "INFO");

            thread::sleep(Duration::from_millis(self.config.restart_delay));
        }
    }

    fn spawn_process(&mut self) -> Result<()> {
        let cmd_str = format!(
            "{} {}",
            self.config.command,
            self.config.args.join(" ")
        );

        self.log(&format!("Executing: {}", cmd_str), "INFO");

        let child = Command::new(&self.config.command)
            .args(&self.config.args)
            .current_dir(&self.config.cwd)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .envs(self.config.env.iter().cloned())
            .spawn()
            .map_err(|e| {
                DaemonError::Process(format!("Failed to spawn '{}': {}", cmd_str, e))
            })?;

        self.child_process = Some(child);
        self.restart_count += 1;

        let msg = format!(
            "Process started (PID: {:?}, restart #{})",
            self.child_process.as_ref().map(|c| c.id()),
            self.restart_count
        );
        self.log(&msg, "SUCCESS");

        Ok(())
    }

    fn wait_for_exit(&mut self) -> Result<()> {
        if let Some(mut child) = self.child_process.take() {
            let status = child.wait().map_err(|e| {
                DaemonError::Process(format!("Failed to wait for process: {}", e))
            })?;

            if status.success() {
                self.log("Process exited normally", "SUCCESS");
            } else {
                let code = status.code().unwrap_or(-1);
                let msg = format!("Process exited with code {}", code);
                self.log(&msg, "WARN");
            }
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.log("Stopping process manager...", "INFO");

        if let Some(mut child) = self.child_process.take() {
            self.log("Sending SIGTERM to child process...", "INFO");

            #[cfg(unix)]
            {
                let pid = child.id() as i32;
                unsafe {
                    libc::kill(pid, libc::SIGTERM);
                }
            }

            #[cfg(not(unix))]
            {
                let _ = child.kill();
            }

            // Wait for graceful exit, then force kill if needed
            let timeout = Duration::from_secs(5);
            let start = std::time::Instant::now();

            while start.elapsed() < timeout {
                if let Ok(Some(_)) = child.try_wait() {
                    self.log("Child process terminated gracefully", "SUCCESS");
                    return Ok(());
                }
                thread::sleep(Duration::from_millis(100));
            }

            self.log("Force killing child process...", "WARN");
            let _ = child.kill();
            let _ = child.wait();
        }

        self.log("Process manager stopped", "SUCCESS");
        Ok(())
    }

    pub fn get_restart_count(&self) -> u32 {
        self.restart_count
    }

    fn log(&self, message: &str, level: &str) {
        use std::io::Write;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let timestamp = format_timestamp(now);
        let log_msg = format!("[{}] [{}] {}", timestamp, level, message);

        // Console output with colors
        let colored_msg = match level {
            "INFO" => format!("\x1b[36m{}\x1b[0m", log_msg),    // cyan
            "SUCCESS" => format!("\x1b[32m{}\x1b[0m", log_msg),  // green
            "WARN" => format!("\x1b[33m{}\x1b[0m", log_msg),     // yellow
            "ERROR" => format!("\x1b[31m{}\x1b[0m", log_msg),    // red
            _ => log_msg.clone(),
        };

        println!("{}", colored_msg);

        // File logging
        if self.config.enable_logging {
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.config.log_file)
            {
                let _ = writeln!(file, "{}", log_msg);
            }
        }
    }
}

fn format_timestamp(secs: u64) -> String {
    // Simple timestamp formatting
    format!("{} UTC", secs)
}
