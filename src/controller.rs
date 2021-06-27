use crate::error::*;
use crate::io::Output;
use crate::pkg::Pkg;

pub struct Controller {
    pub pkg: Pkg,
}

impl Controller {
    pub fn new() -> Result<Self> {
        let pkg = Pkg::init()?;
        let ctrl = Self { pkg };
        Ok(ctrl)
    }

    pub fn write<'a>(&self, output: Output<'a>) {
        // self.pkg.installed
    }
}
