use crate::error::*;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::io::{BufRead, BufReader, BufWriter};
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
        match filename {
            Some(filename) => Self {
                format: parse_output_format(filename),
                filename: Some(filename),
            },
            None => Self {
                format: OutputFormat::Stdout,
                filename: None,
            },
        }
    }

    pub fn write(&self, content: String, force: bool) -> Result<()> {
        match (self.format, self.filename) {
            (OutputFormat::Txt, Some(filename)) => {
                let file = OpenOptions::new()
                    .create_new(!force)
                    .read(force)
                    .write(true)
                    .create(force)
                    .open(Path::new(filename))?;
                let mut writer = BufWriter::new(file);
                writer.write_all(content.as_bytes())?;
                Ok(())
            }
            _ => {
                writeln!(stdout(), "{}", content)?;
                Ok(())
            }
        }
    }
}

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

fn read<'a>(filename: &'a str) -> Result<Vec<String>> {
    let file = OpenOptions::new().read(true).open(Path::new(filename))?;
    let reader = BufReader::new(file);
    let mut file_lines: Vec<String> = Vec::new();
    reader.lines().for_each(|line| {
        let mut line = line.unwrap_or_default();
        let first_char = line.chars().next();
        if first_char != "#".chars().next() && !line.is_empty() {
            let inner_comment_index = line.chars().position(|c| c == '#');
            if let Some(inner_comment_index) = inner_comment_index {
                line = line.chars().skip(0).take(inner_comment_index).collect();
            }
            file_lines.push(line.trim().to_string());
        }
    });
    file_lines.sort();
    file_lines.dedup();
    Ok(file_lines)
}

#[derive(Debug, Clone)]
pub struct Input {
    /// Input format type
    pub format: InputFormat,

    /// Package list to read from
    pub list: Vec<String>,
}

/// TODO: implement input
#[allow(dead_code)]
impl Input {
    pub fn from_file(filename: String) -> Result<Self> {
        let list = read(&filename)?;
        let input = Self {
            format: parse_input_format(&filename),
            list,
        };
        Ok(input)
    }

    pub fn from_list(mut list: Vec<String>) -> Self {
        list.sort();
        list.dedup();
        Self {
            format: InputFormat::Stdin,
            list,
        }
    }

    pub fn append_list(&mut self, list: Vec<String>) {
        let mut list = [&self.list[..], &list[..]].concat();
        list.sort();
        list.dedup();
        &self.set_list(list);
    }

    fn set_list(&mut self, list: Vec<String>) -> &mut Self {
        self.list = list;
        self
    }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn parse_output_format(filename: &str) -> OutputFormat {
    match get_extension_from_filename(filename) {
        Some("txt") => {
            debug!("Using txt extension found in output filename");
            OutputFormat::Txt
        }
        Some(ext) => {
            warn!(
                "Unsupported output format '{}' for output argument '{}', defaulting to plain text",
                ext, filename
            );
            OutputFormat::Txt
        }
        None => {
            info!(
                "No extension found for output argument '{}', defaulting to plain text",
                filename
            );
            OutputFormat::Txt
        }
    }
}

/// TODO: implement input
#[allow(dead_code)]
fn parse_input_format(filename: &str) -> InputFormat {
    match get_extension_from_filename(filename) {
        Some("txt") => {
            debug!("Using txt extension found in input filename");
            InputFormat::Txt
        }
        Some(ext) => {
            warn!(
                "Unsupported output format '{}' for <FILE> argument '{}', defaulting to plain text",
                ext, filename
            );
            InputFormat::Txt
        }
        None => {
            info!(
                "No extension found for output argument '{}', defaulting to plain text",
                filename
            );
            InputFormat::Txt
        }
    }
}
