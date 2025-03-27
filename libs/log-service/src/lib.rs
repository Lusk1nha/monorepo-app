use errors::LogServiceError;
use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt, registry};

pub mod errors;

pub struct LogConfig {
    pub log_directory: PathBuf,
    pub file_prefix: String,
    pub rotation: Rotation,
    pub default_filter: String,
    pub console_filter: Option<String>,
    pub file_filter: Option<String>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            log_directory: PathBuf::from("logs"),
            file_prefix: "app".to_string(),
            rotation: Rotation::HOURLY,
            default_filter: "info".to_string(),
            console_filter: None,
            file_filter: None,
        }
    }
}

/// # Log Service
/// The log service is a simple wrapper around the `tracing` and `tracing-subscriber` crates.
/// It provides a simple way to configure logging for the application.
/// The service is initialized with a `LogConfig` struct that contains the following fields:
/// - `log_directory`: The directory where the log files will be stored.
/// - `file_prefix`: The prefix for the log files.
/// - `rotation`: The rotation strategy for the log files (e.g., hourly, daily, etc.).
/// - `default_filter`: The default log level filter for the application.
/// - `console_filter`: The log level filter for the console output.
/// - `file_filter`: The log level filter for the file output.
/// The service can be initialized with the default configuration by calling `LogService::default()`.
/// The log service can be used to log messages at different levels (e.g., info, debug, error, etc.).
pub struct LogService {
    _work_guard: WorkerGuard,
}

impl LogService {
    pub fn new(config: LogConfig) -> Result<Self, LogServiceError> {
        let file_appender = RollingFileAppender::new(
            config.rotation,
            config.log_directory,
            format!("{}.log", config.file_prefix),
        );

        let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

        let console_filter = Self::build_filter(
            config.console_filter.as_deref(),
            config.default_filter.as_str(),
        )?;

        let file_filter = Self::build_filter(
            config.file_filter.as_deref(),
            config.default_filter.as_str(),
        )?;

        let console_layer = fmt::layer()
            .with_writer(std::io::stdout)
            .with_ansi(true)
            .with_target(true)
            .with_filter(console_filter);

        let file_layer = fmt::layer()
            .with_writer(non_blocking_file)
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(true)
            .with_timer(UtcTime::rfc_3339())
            .with_filter(file_filter);

        registry()
            .with(console_layer)
            .with(file_layer)
            .try_init()
            .map_err(|e| {
                LogServiceError::SubscriberInit(format!("Error initializing subscriber: {}", e))
            })?;

        Ok(Self { _work_guard: guard })
    }

    fn build_filter(
        custom_filter: Option<&str>,
        default_filter: &str,
    ) -> Result<EnvFilter, LogServiceError> {
        match custom_filter {
            Some(filter) => EnvFilter::try_new(filter).map_err(|_| LogServiceError::FilterError),
            None => EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new(default_filter))
                .map_err(|_| LogServiceError::FilterError),
        }
    }

    pub fn default() -> Result<Self, LogServiceError> {
        Self::new(LogConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initiate_log_service() {
        let log_service = LogService::default();
        assert!(log_service.is_ok());
    }

    #[test]
    fn test_build_filter() {
        let filter = LogService::build_filter(None, "info");
        assert!(filter.is_ok());

        let filter = LogService::build_filter(Some("debug"), "info");
        assert!(filter.is_ok());

        let filter = LogService::build_filter(Some("invalid filter"), "filter");
        assert!(filter.is_err());
    }
}
