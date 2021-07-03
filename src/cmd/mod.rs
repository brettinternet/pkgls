use crate::error::Result;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

mod output;
pub mod pacman;

pub type Installed = Vec<String>;
pub type PackageList = Vec<String>;

pub trait PackageManagerCmds {
    fn list_installed(&self) -> Result<Option<Installed>>;

    fn install(&self, package_list: PackageList) -> Result<()>;
}

impl Debug for dyn PackageManagerCmds {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl Display for dyn PackageManagerCmds {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
