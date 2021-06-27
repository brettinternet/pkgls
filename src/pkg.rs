use crate::cmd::Installed;
use crate::error::*;
use crate::manager::Manager;

/// Package manager and associated data
#[derive(Debug)]
pub struct Pkg {
    pub manager: Manager,
    pub installed: Option<Installed>,
}

impl Pkg {
    fn new() -> Result<Self> {
        let pkg = Self {
            manager: Manager::new()?,
            installed: None,
        };

        Ok(pkg)
    }

    pub fn init() -> Result<Pkg> {
        let mut pkg = Self::new()?;

        let installed = pkg.manager.cmd.list_installed()?;
        pkg.set_installed(installed);

        match pkg.installed {
            Some(_) => Ok(pkg),
            None => {
                Err(ErrorKind::PackagesNotFound(pkg.manager.kind.to_string().to_lowercase()).into())
            }
        }
    }

    fn set_installed<'a>(&'a mut self, installed: Option<Vec<String>>) -> &'a mut Self {
        self.installed = installed;
        self
    }
}
