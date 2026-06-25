use std::fmt;

#[derive(Debug)]
pub enum DaemonError {
    Io(std::io::Error),
    Process(String),
    Config(String),
    Daemon(String),
}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DaemonError::Io(e) => write!(f, "IO error: {}", e),
            DaemonError::Process(msg) => write!(f, "Process error: {}", msg),
            DaemonError::Config(msg) => write!(f, "Config error: {}", msg),
            DaemonError::Daemon(msg) => write!(f, "Daemon error: {}", msg),
        }
    }
}

impl From<std::io::Error> for DaemonError {
    fn from(error: std::io::Error) -> Self {
        DaemonError::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, DaemonError>;
