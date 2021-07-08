use super::output::Output;
use super::{FilterOptions, Installed, PackageList, PackageManagerCmds};
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
        let mut install_cmd = Command::new(program);
        install_cmd.arg("-Syu");
        install_cmd.args(&package_list);
        Output::new(&mut install_cmd, program).interact()?;
        Ok(())
    }

    fn post_install_filters(
        &self,
        mut package_list: PackageList,
        options: FilterOptions,
    ) -> Result<PackageList> {
        let FilterOptions { mark_explicit } = options;
        if mark_explicit.is_some() && mark_explicit.unwrap() {
            let mut mark_cmd = Command::new(self.program);
            mark_cmd.arg("-D").arg("--asexplicit");
            mark_cmd.args(&package_list);
        }
        Ok(package_list)
    }
}
