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
    pub output: Option<Output<'a>>,

    /// Input
    pub input: Option<Input>,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Off,
            quiet: false,
            force: false,
            color: true,
            program: None,
            procedure: Procedure::List,
            output: None,
            input: None,
        }
    }
}
