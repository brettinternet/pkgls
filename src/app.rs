use crate::cli::Cli;
use crate::config::Config;
use crate::controller::Controller;
use crate::error::*;
use crate::logger::CliLogger;
use crate::pkg::Pkg;

pub struct App {
    pub controller: Controller,
    pub cli: Cli,
}

impl App {
    fn new(config: Config, cli: Cli) -> Result<Self> {
        let pkg = Pkg::init()?;
        let controller = Controller::new(config, pkg)?;
        let app = Self { controller, cli };
        Ok(app)
    }

    pub fn init(cli: Cli) -> Result<bool> {
        let config = Config {
            quiet: cli.get_quiet(),
            log_level: cli.get_log_level(),
            output_format: cli.get_output_format()?,
            color: cli.color,
        };

        match CliLogger::init(&config) {
            Err(_) => println!("Unable to initialize logging"),
            _ => (),
        };

        let app = Self::new(config, cli)?;

        if let Some(installed) = app.controller.pkg.installed {
            println!("Manager: {}", app.controller.pkg.manager.kind);
            println!("Installed: {:?}", installed);
        }

        Ok(true)
    }
}
