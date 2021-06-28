use crate::error::*;
use crate::io::{Output, OutputFormat};
use crate::pkg::Pkg;
use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::iter::FromIterator;
use std::path::Path;

pub struct Controller {
    pub pkg: Pkg,
}

impl Controller {
    pub fn new() -> Result<Self> {
        let pkg = Pkg::init()?;
        let ctrl = Self { pkg };
        Ok(ctrl)
    }

    pub fn dump<'a>(&self, output: Output<'a>, force: bool) -> Result<()> {
        if let Some(installed) = self.pkg.installed.clone() {
            let out: Result<Box<dyn Write>> = match (output.format, output.filename) {
                (OutputFormat::Txt, Some(filename)) => Ok(OpenOptions::new()
                    .create_new(!force)
                    .read(force)
                    .write(true)
                    .create(force)
                    .open(Path::new(filename))
                    .map(|f| Box::new(f) as Box<dyn Write>)?),
                _ => Ok(Box::new(stdout())),
            };
            let last_index = installed.len() - 1;
            let installed_new_lines = installed
                .into_iter()
                .enumerate()
                .map(|(i, s)| s + if i != last_index { "\n" } else { "" });
            let s = String::from_iter(installed_new_lines);
            writeln!(out?, "{}", s)?;
            Ok(())
        } else {
            Err(ErrorKind::PackagesNotFound(self.pkg.manager.get_kind_lowercase()).into())
        }
    }
}
