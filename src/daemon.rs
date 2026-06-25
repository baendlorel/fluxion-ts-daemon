use crate::config::Config;
use crate::error::{DaemonError, Result};
use crate::process::ProcessManager;
use std::fs::File;
use std::os::unix::ffi::OsStrExt;

pub struct Daemon {
    config: Config,
    process_manager: Option<ProcessManager>,
}

impl Daemon {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_manager: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.log("=== Fluxion Daemon Starting ===", "INFO");
        self.log(&format!("Working directory: {:?}", self.config.cwd), "INFO");
        self.log(&format!("Log file: {:?}", self.config.log_file), "INFO");

        // Create daemon
        self.create_daemon()?;

        self.log("Successfully daemonized", "SUCCESS");

        // Start process manager
        let mut process_manager = ProcessManager::new(self.config.clone());
        self.process_manager = Some(process_manager);

        // Run the process manager
        if let Some(ref mut manager) = self.process_manager {
            manager.start()?;
        }

        Ok(())
    }

    #[cfg(unix)]
    fn create_daemon(&self) -> Result<()> {
        use std::os::unix::io::AsRawFd;

        // Fork for the first time
        unsafe {
            match libc::fork() {
                -1 => return Err(DaemonError::Daemon("First fork failed".to_string())),
                0 => {
                    // Child process
                }
                _ => {
                    // Parent process exits
                    libc::_exit(0);
                }
            }

            // Create new session
            if libc::setsid() == -1 {
                return Err(DaemonError::Daemon("setsid failed".to_string()));
            }

            // Fork for the second time
            match libc::fork() {
                -1 => return Err(DaemonError::Daemon("Second fork failed".to_string())),
                0 => {
                    // Child process
                }
                _ => {
                    // Parent process exits
                    libc::_exit(0);
                }
            }

            // Change working directory
            let cwd_path = self.config.cwd.as_os_str().as_bytes();
            libc::chdir(cwd_path.as_ptr() as *const i8);

            // Reset file mode mask
            libc::umask(0);

            // Close all open file descriptors
            let maxfd = unsafe { libc::sysconf(libc::_SC_OPEN_MAX) } as i32;
            for fd in 0..maxfd {
                if fd != libc::STDIN_FILENO
                    && fd != libc::STDOUT_FILENO
                    && fd != libc::STDERR_FILENO
                {
                    libc::close(fd);
                }
            }

            // Redirect stdin, stdout, stderr to /dev/null
            let dev_null = File::open("/dev/null")?;
            let dev_null_fd = dev_null.as_raw_fd();

            libc::dup2(dev_null_fd, libc::STDIN_FILENO);
            libc::dup2(dev_null_fd, libc::STDOUT_FILENO);
            libc::dup2(dev_null_fd, libc::STDERR_FILENO);
        }

        Ok(())
    }

    #[cfg(not(unix))]
    fn create_daemon(&self) -> Result<()> {
        // On non-Unix systems, just log that we're running as a regular process
        self.log(
            "Daemonization not supported on this platform, running as regular process",
            "WARN",
        );
        Ok(())
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
            "INFO" => format!("\x1b[36m{}\x1b[0m", log_msg), // cyan
            "SUCCESS" => format!("\x1b[32m{}\x1b[0m", log_msg), // green
            "WARN" => format!("\x1b[33m{}\x1b[0m", log_msg), // yellow
            "ERROR" => format!("\x1b[31m{}\x1b[0m", log_msg), // red
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
    format!("{} UTC", secs)
}
