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

/// TODO: implement input
#[allow(dead_code)]
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

/// TODO: implement input
#[allow(dead_code)]
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
    match (filename, get_extension_from_filename(filename)) {
        (Some(_), Some("txt")) => {
            debug!("Using txt extension found in output filename");
            OutputFormat::Txt
        }
        (Some(filename), Some(ext)) => {
            warn!(
                "Unsupported output format '{}' for output argument '{}', defaulting to plain text",
                ext, filename
            );
            OutputFormat::Txt
        }
        (Some(filename), None) => {
            info!(
                "No extension found for output argument '{}', defaulting to plain text",
                filename
            );
            OutputFormat::Txt
        }
        (None, _) => {
            debug!("Missing output filename extension, defaulting to stdout");
            OutputFormat::Stdout
        }
    }
}

/// TODO: implement input
#[allow(dead_code)]
fn parse_input_format(filename: Option<&str>) -> InputFormat {
    match (filename, get_extension_from_filename(filename)) {
        (Some(_), Some("txt")) => {
            debug!("Using txt extension found in input filename");
            InputFormat::Txt
        }
        (Some(filename), Some(ext)) => {
            warn!(
                "Unsupported output format '{}' for <FILE> argument '{}', defaulting to plain text",
                ext, filename
            );
            InputFormat::Txt
        }
        (Some(filename), None) => {
            info!(
                "No extension found for output argument '{}', defaulting to plain text",
                filename
            );
            InputFormat::Txt
        }
        (None, _) => {
            debug!("Missing input filename extension, defaulting to stdin");
            InputFormat::Stdin
        }
    }
}
