use crate::app::Procedure;
use crate::io::{Input, Output};
use crate::logger::filter_level_occurences;
use clap::{App, AppSettings, Arg, ArgMatches, Values};
use log::LevelFilter;
use std::env;
use std::path::Path;

/// Cli input parsed by Clap and associated input interpreters
#[derive(Debug)]
pub struct Cli {
    /// Clap argument matches
    matches: ArgMatches,

    /// If color is supported
    pub color: bool,
}

impl Cli {
    /// TODO: support -u uninstall, -i install, -e explicitly installed filter, -v use versions
    pub fn new() -> Self {
        let color = env::var_os("NO_COLOR").is_none();
        let cli_color_setting = if color {
            AppSettings::ColoredHelp
        } else {
            AppSettings::ColorNever
        };

        let app = App::new("pkgls")
            .version("0.1.0")
            // .license("MIT") // unreleased
            .global_setting(cli_color_setting)
            .global_setting(AppSettings::UnifiedHelpMessage)
            .setting(AppSettings::ArgsNegateSubcommands)
            .setting(AppSettings::DisableHelpSubcommand)
            .setting(AppSettings::VersionlessSubcommands)
            .max_term_width(100)
            .author("Brett (https://github.com/brettinternet/pkgls)")
            .about("List installed packages")
            .arg(
                Arg::new("program")
                    .short('p')
                    .long("program")
                    .takes_value(true)
                    .possible_values(&["pacman"])
                    .about("Explicitly set which package manager to use"),
            )
            .arg(
                Arg::new("log")
                    .short('l')
                    .long("log")
                    .takes_value(false)
                    .multiple_occurrences(true)
                    .conflicts_with("quiet")
                    .about("Increment a log level"),
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .conflicts_with("log")
                    .about("Silence stdout and stderr"),
            )
            .subcommand(
                App::new("list")
                    .alias("show")
                    .about("List installed packages or save to file")
                    .arg(
                        Arg::new("output")
                            .index(1)
                            .about("Filename to write package names"),
                    )
                    .arg(
                        Arg::new("force")
                            .short('f')
                            .long("force")
                            .requires("output")
                            .about("Force overwrite the output if it already exists"),
                    )
                    .arg(
                        Arg::new("input")
                            .short('i')
                            .long("input")
                            .about("Packages to filter output from a file")
                            .multiple(true)
                            .takes_value(true),
                    ),
            )
            .subcommand(
                App::new("install")
                    .alias("add")
                    .about("Install packages from input or a file")
                    .arg(
                        Arg::new("packages")
                            .index(1)
                            .about("Package names to install")
                            .multiple(true)
                            .min_values(1)
                            .conflicts_with("input"),
                    )
                    .arg(
                        Arg::new("input")
                            .short('i')
                            .long("input")
                            .about("Packages to read from a file")
                            .conflicts_with("packages")
                            .multiple(true)
                            .takes_value(true),
                    ),
            );

        let matches = app.get_matches();
        Self { matches, color }
    }

    /// Subcommand procedure
    ///
    /// Derive the procedure from the subcommands
    pub fn get_procedure(&self) -> Procedure {
        if self.matches.is_present("list") {
            Procedure::List
        } else if self.matches.is_present("install") {
            Procedure::Install
        } else {
            info!("Running 'list' subcommand by default");
            Procedure::List
        }
    }

    /// Output filename
    pub fn get_output(&self) -> Option<Output> {
        if let Some(list_matches) = self.matches.subcommand_matches("list") {
            Some(Output::new(list_matches.value_of("output")))
        } else {
            None
        }
    }

    /// Collect input from multiple or single file input
    ///
    /// Used for both install and list inputs
    fn get_file_input(&self, filenames: Values) -> Option<Input> {
        let filenames: Vec<String> = filenames.map(str::to_string).collect();
        if let Some(first_filename) = filenames.first() {
            let input = Input::from_file(first_filename.to_string()).ok();
            let mut packages = Vec::new();
            for s in &filenames {
                let subinput = Input::from_file(s.to_string()).ok();
                if let Some(mut subinput) = subinput {
                    packages.append(&mut subinput.list);
                    continue;
                } else {
                    error!("Unable to read file '{}'", s);
                }
            }
            if let Some(mut first_input) = input.clone() {
                first_input.append_list(packages);
                Some(first_input)
            } else {
                error!("Unable to read file '{}'", first_filename);
                input
            }
        } else {
            warn!("No files received in input");
            None
        }
    }

    /// Input filename or packages to filter installed list
    pub fn get_list_input(&self) -> Option<Input> {
        if let Some(list_matches) = self.matches.subcommand_matches("list") {
            if let Some(filenames) = list_matches.values_of("input") {
                self.get_file_input(filenames)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Input filename or packages
    pub fn get_install_input(&self) -> Option<Input> {
        if let Some(list_matches) = self.matches.subcommand_matches("install") {
            if let Some(list) = list_matches.values_of("packages") {
                let list: Vec<String> = list.map(str::to_string).collect();
                let mut packages = Vec::new();
                for s in &list {
                    // Probably shouldn't let users cheat by allowing packages or filename input here...
                    if Path::new(s).exists() {
                        let subinput = Input::from_file(s.to_string()).ok();
                        if let Some(mut subinput) = subinput {
                            packages.append(&mut subinput.list);
                            continue;
                        }
                    };
                    packages.push(s.to_string());
                }
                if packages.is_empty() {
                    warn!("No packages received in input");
                }
                let input = Input::from_list(packages);
                Some(input)
            } else if let Some(filenames) = list_matches.values_of("input") {
                self.get_file_input(filenames)
            } else {
                warn!("No files received in input");
                None
            }
        } else {
            warn!("No files received in input");
            None
        }
    }

    /// Force
    ///
    /// Force overwriting target file
    ///
    /// bool whether force is requested
    pub fn get_force(&self) -> bool {
        if let Some(list_matches) = self.matches.subcommand_matches("list") {
            list_matches.is_present("force")
        } else {
            false
        }
    }

    /// Quiet
    ///
    /// Silence all output
    ///
    /// bool whether output should be silenced
    pub fn get_quiet(&self) -> bool {
        self.matches.is_present("quiet")
    }

    /// Log level
    ///
    /// Supported log levels are off, debug, info, warn and error
    ///
    /// Default log level is "off"
    pub fn get_log_level(&self) -> LevelFilter {
        if self.get_quiet() {
            LevelFilter::Off
        } else {
            let count = self.matches.occurrences_of("log");
            filter_level_occurences(count)
        }
    }

    /// Package manager program
    pub fn get_program(&self) -> Option<&str> {
        self.matches.value_of("program")
    }
}
