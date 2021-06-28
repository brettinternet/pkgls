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
        #[cfg(debug_assertions)]
        println!("config: {:?}", config);
        match CliLogger::init(&config) {
            Err(_) => println!("Failed to initialize logging"),
            _ => (),
        };
        let mut controller = Controller::new()?;
        if let Some(program) = config.program {
            controller.pkg.manager.configure_kind(program)?;
        }
        let app = Self { controller, config };
        Ok(app)
    }

    pub fn init(&self) -> Result<bool> {
        self.controller.write(self.config.output);

        if let Some(installed) = &self.controller.pkg.installed {
            println!("Manager: {}", self.controller.pkg.manager.kind);
            println!("Installed: {:?}", installed);
        }

        Ok(true)
    }
}
