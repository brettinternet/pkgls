#[macro_use]
extern crate log;

use crate::app::{App, Procedure};
use crate::cli::Cli;
use crate::error::{default_error_handler, Result};
use config::Config;
use io::{Input, Output};
use std::process;

mod app;
mod cli;
mod cmd;
mod config;
mod controller;
mod error;
mod io;
mod logger;
mod manager;
mod pkg;

fn run(config: Config) -> Result<bool> {
    App::new(config)?.init()
}

fn main() {
    let cli = Cli::new();
    let quiet = cli.get_quiet();
    let procedure = cli.get_procedure();
    let (input, output): (Option<Input>, Option<Output>) = match procedure {
        Procedure::List => (cli.get_list_input(), cli.get_output()),
        Procedure::Install => (cli.get_install_input(), None),
    };
    let config = Config {
        log_level: cli.get_log_level(),
        quiet,
        force: cli.get_force(),
        color: cli.color,
        program: cli.get_program(),
        procedure,
        output,
        input,
    };
    #[cfg(debug_assertions)]
    println!("config: {:?}", config);

    match run(config) {
        Err(error) => {
            if !quiet {
                let stderr = std::io::stderr();
                default_error_handler(&error, &mut stderr.lock());
            }
            process::exit(1)
        }
        Ok(false) => process::exit(1),
        Ok(true) => process::exit(0),
    }
}
