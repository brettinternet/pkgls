use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;

/// TODO: support toml, yml, json
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    /// Print output to console
    Stdout,

    // Format to txt file
    Txt,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Output<'a> {
    /// Output format type
    pub format: OutputFormat,

    /// Target file
    pub filename: Option<&'a str>,
}

impl<'a> Output<'a> {
    pub fn new(filename: Option<&'a str>) -> Self {
        Self {
            format: parse_output_format(filename),
            filename,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InputFormat {
    /// Print output to console
    Stdin,

    // Format to txt file
    Txt,
}

impl Display for InputFormat {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Input<'a> {
    /// Input format type
    pub format: InputFormat,

    /// Files to read from
    pub filename: Option<&'a str>,
}

impl<'a> Input<'a> {
    pub fn new(filename: Option<&'a str>) -> Self {
        Self {
            format: parse_input_format(filename),
            filename,
        }
    }
}

fn get_extension_from_filename(filename: Option<&str>) -> Option<&str> {
    if let Some(filename) = filename {
        Path::new(filename).extension().and_then(OsStr::to_str)
    } else {
        None
    }
}

fn parse_output_format(filename: Option<&str>) -> OutputFormat {
    match get_extension_from_filename(filename) {
        Some("txt") => {
            debug!("Using txt extension found in output filename");
            OutputFormat::Txt
        }
        None => {
            debug!("Missing output filename extension, defaulting to stdout");
            OutputFormat::Stdout
        }
        Some(ext) => {
            warn!("Unsupported output format '{}' for <FILE> argument", ext);
            OutputFormat::Txt
        }
    }
}

fn parse_input_format(filename: Option<&str>) -> InputFormat {
    match get_extension_from_filename(filename) {
        Some("txt") => {
            debug!("Using txt extension found in input filename");
            InputFormat::Txt
        }
        None => {
            debug!("Missing input filename extension, defaulting to stdin");
            InputFormat::Stdin
        }
        Some(ext) => {
            warn!("Unsupported output format '{}' for <FILE> argument", ext);
            InputFormat::Txt
        }
    }
}
