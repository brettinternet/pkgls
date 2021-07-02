use crate::config::Config;
use crate::controller::Controller;
use crate::error::*;
use crate::logger::CliLogger;

#[derive(Debug)]
pub enum Procedure {
    List,
}

pub struct App<'a> {
    pub controller: Controller,
    pub config: Config<'a>,
}

impl<'a> App<'a> {
    pub fn new(config: Config<'a>) -> Result<Self> {
        match CliLogger::init(&config) {
            Err(_) => println!("Failed to initialize logging"),
            _ => (),
        };
        #[cfg(debug_assertions)]
        debug!("config: {:?}", config);
        let mut controller = Controller::new()?;
        if let Some(program) = config.program {
            controller.pkg.manager.configure_kind(program)?;
        }
        let app = Self { controller, config };
        Ok(app)
    }

    pub fn init(&self) -> Result<bool> {
        match self.config.procedure {
            Procedure::List => {
                if let Some(output) = self.config.output {
                    self.controller.dump(output, self.config.force)?;
                    Ok(true)
                } else {
                    error!("Missing output file from list subcommand");
                    Ok(false)
                }
            }
        }
    }
}
