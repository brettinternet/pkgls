use crate::config::Config;
use crate::error::*;
use crate::pkg::Pkg;

pub struct Controller {
    pub config: Config,
    pub pkg: Pkg,
}

impl Controller {
    pub fn new(config: Config, pkg: Pkg) -> Result<Self> {
        let ctrl = Self { config, pkg };
        Ok(ctrl)
    }
}
