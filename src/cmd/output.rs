use crate::error::*;
use std::io::{BufRead, BufReader};
use std::process::{ChildStderr, ChildStdout, Command, Stdio};

pub struct Output<'a> {
    cmd: &'a mut Command,
    program: &'static str,
}

impl<'a> Output<'a> {
    pub fn new(cmd: &'a mut Command, program: &'static str) -> Self {
        Self { cmd, program }
    }

    pub fn read(&'a mut self) -> Result<Option<Vec<String>>> {
        self.cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        // Wait for https://github.com/rust-lang/rust/issues/44434
        // to extract fields from Command builder
        // let program = self.cmd.get_program();

        // TODO: consolidate matches
        match self.cmd.spawn() {
            Ok(ref mut output) => match (output.stdout.take(), output.stderr.take()) {
                (Some(stdout), Some(stderr)) => {
                    match (self.read_stdout(stdout), self.read_stderr(stderr)) {
                        (Some(out), Some(err)) => {
                            error!("{}", err);
                            Ok(Some(out))
                        }
                        (Some(out), None) => Ok(Some(out)),
                        (None, Some(err)) => Err(ErrorKind::InterruptedManager(err).into()),
                        _ => Ok(None),
                    }
                }
                (None, Some(stderr)) => Err(ErrorKind::InterruptedManager(
                    self.read_stderr(stderr)
                        .unwrap_or("Unable to read stderr of command".to_string()),
                )
                .into()),
                _ => Ok(None),
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                error!("'{}' was not found", self.program);
                Ok(None)
            }
            Err(_) => {
                error!("Could not spawn command for '{}'", self.program);
                Ok(None)
            }
        }
    }

    fn read_stdout(&self, stdout: ChildStdout) -> Option<Vec<String>> {
        let out = BufReader::new(stdout);
        let mut out_lines: Vec<String> = Vec::new();
        out.lines().for_each(|line| {
            out_lines.push(line.unwrap());
        });
        if out_lines.is_empty() {
            None
        } else {
            Some(out_lines)
        }
    }

    fn read_stderr(&self, stderr: ChildStderr) -> Option<String> {
        let err = BufReader::new(stderr);
        let mut err_lines = String::new();
        err.lines().for_each(|line| {
            err_lines.push_str(line.unwrap().as_str());
        });
        if err_lines.is_empty() {
            None
        } else {
            Some(err_lines)
        }
    }
}
