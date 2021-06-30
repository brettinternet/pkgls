use crate::app::Procedure;
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

    /// Package manager program
    pub program: Option<&'a str>,

    /// Procedural action to commit
    pub procedure: Procedure,

    /// Output
    pub output: Output<'a>,

    /// Input
    pub input: Option<Input<'a>>,
}
