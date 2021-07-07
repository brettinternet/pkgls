use crate::cmd::{Installed, PackageList};
use crate::error::*;
use crate::manager::Manager;

/// Package manager and associated data
#[derive(Debug)]
pub struct Pkg {
    /// Interface for a package manager and associated cmds
    pub manager: Manager,

    /// List of installed packages evaluated immediately at runtime
    pub installed: Option<Installed>,

    /// List of missing packages evaluted from input lists
    pub missing: Option<PackageList>,
}

impl Pkg {
    fn new() -> Result<Self> {
        let pkg = Self {
            manager: Manager::new()?,
            installed: None,
            missing: None,
        };

        Ok(pkg)
    }

    pub fn init() -> Result<Pkg> {
        let mut pkg = Self::new()?;

        let installed = pkg.manager.cmd.list_installed()?;
        pkg.set_installed(installed);

        match pkg.installed {
            Some(_) => Ok(pkg),
            None => Err(ErrorKind::PackagesNotFound(pkg.manager.get_kind_lowercase()).into()),
        }
    }

    pub fn update_installed(&mut self) -> Result<()> {
        let installed = self.manager.cmd.list_installed()?;
        self.set_installed(installed);
        Ok(())
    }

    fn set_installed<'a>(&'a mut self, installed: Option<Vec<String>>) -> &'a mut Self {
        self.installed = installed;
        self
    }

    fn set_missing<'a>(&'a mut self, list: Vec<String>) -> &'a mut Self {
        let missing = if let Some(installed) = self.installed.clone() {
            list.into_iter()
                .filter(|item| !installed.contains(item))
                .collect()
        } else {
            list
        };
        self.missing = Some(missing);
        self
    }

    pub fn install_missing(&mut self, list: Vec<String>) -> Result<()> {
        self.set_missing(list.clone());
        match self.missing.clone() {
            Some(missing) if !missing.is_empty() => match self.manager.cmd.install(missing.clone())
            {
                Ok(_) => {
                    info!("Successfully installed packages: {}", missing.join(" "));
                    Ok(())
                }
                Err(_) => {
                    debug!(
                        "Checking installed packages to determine which packages failed to install"
                    );
                    self.update_installed()?;
                    self.set_missing(list.clone());
                    match self.missing.clone() {
                        Some(missing) if missing.len() == list.len() => {
                            debug!("No missing packages were installed");
                            Err(
                                ErrorKind::InterruptedManager("Installation cancelled".into())
                                    .into(),
                            )
                        }
                        Some(missing) if !missing.is_empty() => {
                            warn!("Cannot find missing packages after the attempted installation");
                            Err(ErrorKind::FailedManager(format!(
                                "Failed to install packages: {}",
                                missing.join(" ")
                            ))
                            .into())
                        }
                        _ => {
                            warn!("Cannot find missing packages");
                            Err(ErrorKind::InterruptedManager("Installation failed".into()).into())
                        }
                    }
                }
            },
            _ => {
                warn!("No missing packages found to install");
                Ok(())
            }
        }
    }
}
