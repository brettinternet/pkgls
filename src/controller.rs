use crate::error::*;
use crate::io::{Input, Output};
use crate::pkg::Pkg;

pub struct Controller {
    pub pkg: Pkg,
}

/// TODO: print deps count, explicit install count, total, etc
impl Controller {
    pub fn new() -> Result<Self> {
        let pkg = Pkg::init()?;
        let ctrl = Self { pkg };
        Ok(ctrl)
    }

    pub fn dump<'a>(&self, input: &Option<Input>, output: &Output<'a>, force: bool) -> Result<()> {
        if let Some(installed) = self.pkg.installed.clone() {
            let installed = if let Some(input) = input {
                installed
                    .into_iter()
                    .filter(|item| !input.list.contains(item))
                    .collect()
            } else {
                installed
            };
            let last_index = installed.len() - 1;
            let installed_new_lines = installed
                .into_iter()
                .enumerate()
                .map(|(i, s)| s + if i != last_index { "\n" } else { "" });
            let content = installed_new_lines.collect::<String>();
            output.write(content, force)
        } else {
            Err(ErrorKind::PackagesNotFound(self.pkg.manager.get_kind_lowercase()).into())
        }
    }

    pub fn install(&mut self, input: &Input) -> Result<()> {
        self.pkg.install_missing(input.list.clone())?;
        Ok(())
    }
}
