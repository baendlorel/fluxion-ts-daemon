use crate::error::{DaemonError, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub command: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub restart_delay: u64,
    pub max_restarts: Option<u32>,
    pub enable_logging: bool,
    pub log_file: PathBuf,
    pub env: Vec<(String, String)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            command: "tsx".to_string(),
            args: vec!["main.ts".to_string()],
            cwd: PathBuf::from("."),
            restart_delay: 1000,
            max_restarts: None,
            enable_logging: true,
            log_file: PathBuf::from("daemon.log"),
            env: Vec::new(),
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.command.is_empty() {
            return Err(DaemonError::Config("Command cannot be empty".to_string()));
        }
        Ok(())
    }
}
