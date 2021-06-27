use crate::error::*;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

pub struct Output<'a> {
    cmd: &'a mut Command,
    program: &'static str,
}

impl<'a> Output<'a> {
    pub fn new(cmd: &'a mut Command, program: &'static str) -> Self {
        Self { cmd, program }
    }

    pub fn read(&'a mut self) -> Result<Option<Vec<String>>> {
        self.cmd
            // .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Wait for https://github.com/rust-lang/rust/issues/44434
        // to extract fields from Command builder
        // let program = self.cmd.get_program();

        match self.cmd.spawn() {
            Ok(ref mut output) => {
                if let Some(out) = output.stdout.take() {
                    let out = BufReader::new(out);
                    let mut lines: Vec<String> = Vec::new();
                    out.lines().for_each(|line| {
                        let line = line.unwrap();
                        println!("line: {}", line);
                        lines.push(line);
                    });
                    // let out = self.read_output(output)?;
                    // TODO: format output
                    // let data = BufReader::new(output.stdout);
                    // Ok(out)
                    Ok(Some(lines))
                } else {
                    Ok(None)
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                error!("'{}' was not found", self.program);
                Ok(None)
            }
            Err(_) => {
                error!("Could not spawn command for '{}'", self.program);
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    // /// https://doc.rust-lang.org/std/process/struct.Command.html
    // fn read_output(&self, output: Child) -> Result<Option<&str>> {
    //     match (output.stdout, output.stderr) {
    //         (Some(out), _) => {
    //             self.log_stderr(output);

    //             #[cfg(debug_assertions)]
    //             debug!("Packages found from {}, output: {:?}", self.program, out);
    //             let out_str = String::new();
    //             out_str = String::from_utf8(out)?;

    //             Ok(self.parse_std(out)?)
    //         }
    //         (None, Some(stderr)) => {
    //             self.log_stderr(output);
    //             let err = self.parse_std(stderr);
    //             Err(ErrorKind::InterruptionStdErr(err).into())
    //         }
    //         (None, None) => Ok(None),
    //     }
    // }

    // fn log_stderr(&self, output: Child) {
    //     if let Some(err) = output.stderr {
    //         if let Some(stdin) = output.stdin {
    //             let input = self.parse_std(stdin)?;
    //             let input = String::from_utf8_lossy(input);
    //             error!("Command '{}' returned an error: {:?}", input, err);
    //         } else {
    //             error!("Command returned an error: {:?}", err);
    //         }
    //     }
    // }

    // fn parse_std(self, std: &[u8]) -> Result<&str> {
    //     let result = std::str::from_utf8(std)?;
    //     Ok(result)
    // }
}
