use crate::config::Config;
use crate::controller::Controller;
use crate::error::*;
use crate::logger::CliLogger;

#[derive(Debug)]
pub enum Procedure {
    List,
    Install,
}

pub struct App<'a> {
    pub controller: Controller,
    pub config: Config<'a>,
}

impl<'a> App<'a> {
    pub fn new(config: Config<'a>) -> Result<Self> {
        if CliLogger::init(&config.log_level).is_err() {
            println!("Failed to initialize logging");
        }
        #[cfg(debug_assertions)]
        debug!("config: {:?}", config);
        let mut controller = Controller::new()?;
        if let Some(program) = config.program {
            controller.pkg.manager.configure_kind(program)?;
        }
        let app = Self { controller, config };
        Ok(app)
    }

    pub fn init(&mut self) -> Result<bool> {
        match self.config.procedure {
            Procedure::List => {
                if let Some(output) = &self.config.output {
                    self.controller
                        .dump(&self.config.input, output, self.config.force)?;
                    Ok(true)
                } else {
                    error!("Missing output from list subcommand");
                    Ok(false)
                }
            }
            Procedure::Install => {
                if let Some(input) = &self.config.input {
                    self.controller.install(input)?;
                    Ok(true)
                } else {
                    error!("Missing input from install subcommand");
                    Ok(false)
                }
            }
        }
    }
}
