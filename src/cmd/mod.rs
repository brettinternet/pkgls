use crate::error::Result;

mod output;
pub mod pacman;

pub type Installed = Vec<String>;
pub type PackageList = Vec<String>;

pub trait PackageManagerCmds {
    fn list_installed(&self) -> Result<Option<Installed>>;

    fn install(&self, package_list: PackageList) -> Result<()>;
}
}
