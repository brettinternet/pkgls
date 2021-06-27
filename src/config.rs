use log::LevelFilter;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    /// Print output to console
    Stdout,
    // Format to txt file
    // Txt,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Config {
    /// Log level
    pub log_level: LevelFilter,

    /// Silence all output
    pub quiet: bool,

    /// Output format type
    pub output_format: OutputFormat,

    /// If output should be colorized
    pub color: bool,
}
