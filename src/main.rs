#[macro_use]
extern crate log;
use crate::app::App;
use crate::cli::Cli;
use crate::error::default_error_handler;
use std::process;

mod app;
mod cli;
mod cmd;
mod config;
mod controller;
mod error;
mod logger;
mod manager;
mod pkg;

fn main() {
    // let cli = Cli::new();
    let quiet = false;
    let app = App::init(Cli::new());

    match app {
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
