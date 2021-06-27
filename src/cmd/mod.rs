use crate::error::Result;

mod output;
pub mod pacman;

pub type Installed = Vec<String>;

pub trait ListInstalled {
    fn list_installed(&self) -> Result<Option<Installed>>;
}
