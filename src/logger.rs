use env_logger::filter::{Builder, Filter};
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::boxed::Box;

/// Expected environment variable to set log level
const LOG_LEVEL_ENV: &str = "PKGLS_LOG";

/// Logger options
pub struct CliLogger {
    inner: Filter,
}

/// TODO: use colored output from https://github.com/env-logger-rs/env_logger
impl CliLogger {
    fn new(log_level: &LevelFilter) -> Self {
        let mut builder = Builder::new();

        // Use println! macro since logger isn't available yet
        if let Ok(ref filter) = std::env::var(LOG_LEVEL_ENV) {
            builder.parse(filter);

            if filter == "debug" {
                println!(
                    "Applying log level from environment {}={}",
                    LOG_LEVEL_ENV, filter
                );
            }

            // Advise the user of log level origin if debug arg is applied and env var is used
            if *log_level == LevelFilter::Debug {
                println!("Using log level from environment instead of argument");
            }
        } else {
            builder.filter_level(*log_level);
        }

        Self {
            inner: builder.build(),
        }
    }

    pub fn init(log_level: &LevelFilter) -> Result<(), SetLoggerError> {
        let logger = Self::new(log_level);
        let level = logger.inner.filter();

        #[cfg(debug_assertions)]
        println!("Log level: {}", level);

        log::set_max_level(level);
        log::set_boxed_logger(Box::new(logger))
    }
}

impl Log for CliLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.inner.matches(record) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

/// No flag -> off
///
/// Single flag -> error
///
/// Double flag -> warn
///
/// Triple flag -> info
///
/// \>3 flag -> debug
pub fn filter_level_occurences(count: u64) -> LevelFilter {
    match count {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    }
}

#[cfg(test)]
mod tests {
    use super::CliLogger;
    use log::LevelFilter;

    #[test]
    fn inits_logger() {
        let log_init = CliLogger::init(&LevelFilter::Debug);
        assert!(log_init.is_ok(), "Failed to initialize logger");
    }
}
