use log::LevelFilter;

use crate::io::{Input, Output};

/// App interface
#[derive(Debug)]
pub struct Config<'a> {
    /// Log level
    pub log_level: LevelFilter,

    /// Silence all output
    pub quiet: bool,

    /// Whether to force writing target file
    pub force: bool,

    /// If output should be colorized
    pub color: bool,

    /// Output
    pub output: Output<'a>,

    /// Input
    pub input: Option<Input<'a>>,
}
