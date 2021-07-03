use super::output::Output;
use super::{Installed, PackageList, PackageManagerCmds};
use crate::error::*;
use std::process::Command;

/// Pacman for Arch Linux
///
/// Docs: https://wiki.archlinux.org/title/Pacman
#[derive(Debug)]
pub struct PacmanCmd {
    program: &'static str,
}

impl PacmanCmd {
    pub fn new() -> Self {
        Self { program: "pacman" }
    }
}

impl PackageManagerCmds for PacmanCmd {
    fn list_installed(&self) -> Result<Option<Installed>> {
        let program = self.program;
        let mut cmd = Command::new(program);
        cmd.arg("-Qeq");
        Output::new(&mut cmd, program).read_packages()
    }

    fn install(&self, package_list: PackageList) -> Result<()> {
        let program = self.program;
        let mut cmd = Command::new(program);
        cmd.arg("-Syu");
        cmd.args(&package_list);
        Output::new(&mut cmd, program).interact()?;
        Ok(())
    }
}
