use super::output::Output;
use super::{Installed, ListInstalled};
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

impl ListInstalled for PacmanCmd {
    fn list_installed(&self) -> Result<Option<Installed>> {
        let program = self.program;
        let mut cmd = Command::new(program);
        cmd.arg("-Qeq");
        Output::new(&mut cmd, program).read()
    }
}
