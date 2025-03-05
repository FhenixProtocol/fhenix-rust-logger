use anyhow::Result;
use serde::Deserialize;
use strum::Display;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Debug, Clone, Copy, Deserialize, Display, Default)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

const fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggerConfig {
    #[serde(default)]
    pub level: LogLevel,
    #[serde(default = "default_true")]
    pub show_thread_id: bool,
    #[serde(default = "default_true")]
    pub show_thread_name: bool,
    #[serde(default = "default_true")]
    pub show_file: bool,
    #[serde(default = "default_true")]
    pub show_line_number: bool,
    #[serde(default = "default_true")]
    pub show_target: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::default(),
            show_thread_id: true,
            show_thread_name: true,
            show_file: true,
            show_line_number: true,
            show_target: true,
        }
    }
}

/// Initialize the logger with the given configuration
///
/// # Example
/// ```rust
/// use logger::{init_logger, LoggerConfig, LogLevel};
///
/// let config = LoggerConfig {
///     level: LogLevel::Debug,
///     show_file: false,
///     ..Default::default()
/// };
///
/// init_logger("my-service", &config).expect("Failed to initialize logger");
/// ```
pub fn init_logger(service_name: &str, config: &LoggerConfig) -> Result<()> {
    let filter = EnvFilter::new(format!(
        "{},{}={}",
        config.level, service_name, config.level
    ));

    // Set up the subscriber, will return error if already initialized
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_thread_ids(config.show_thread_id)
                .with_thread_names(config.show_thread_name)
                .with_file(config.show_file)
                .with_line_number(config.show_line_number)
                .with_target(config.show_target),
        )
        .try_init()?;

    Ok(())
}

/// Initialize the logger with default configuration
///
/// This is equivalent to calling `init_logger(service_name, &LoggerConfig::default())`
pub fn init_default_logger(service_name: &str) -> Result<()> {
    init_logger(service_name, &LoggerConfig::default())
}
