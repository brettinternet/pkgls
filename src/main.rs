#[macro_use]
extern crate log;

use crate::app::App;
use crate::cli::Cli;
use crate::error::default_error_handler;
use config::Config;
use io::Output;
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

fn main() {
    let cli = Cli::new();
    let quiet = cli.get_quiet();
    let options = Config {
        log_level: cli.get_log_level(),
        quiet,
        force: cli.get_force(),
        color: cli.color,
        output: Output::new(cli.get_file()),
        input: None, // Not supported yet
    };

    let app = App::new(options).unwrap();

    match app.init() {
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
