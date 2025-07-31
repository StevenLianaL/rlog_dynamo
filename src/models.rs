use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}
impl LogLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        }
    }
}
