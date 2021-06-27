use crate::config::Config;
use crate::controller::Controller;
use crate::error::*;
use crate::logger::CliLogger;

pub struct App<'a> {
    pub controller: Controller,
    pub config: Config<'a>,
}

impl<'a> App<'a> {
    pub fn new(config: Config<'a>) -> Result<Self> {
        let controller = Controller::new()?;
        let app = Self { controller, config };
        Ok(app)
    }

    pub fn init(&self) -> Result<bool> {
        self.init_logger();

        self.controller.write(self.config.output);

        if let Some(installed) = &self.controller.pkg.installed {
            println!("Manager: {}", self.controller.pkg.manager.kind);
            println!("Installed: {:?}", installed);
        }

        Ok(true)
    }

    fn init_logger(&self) {
        match CliLogger::init(&self.config) {
            Err(_) => println!("Failed to initialize logging"),
            _ => (),
        };
    }
}
